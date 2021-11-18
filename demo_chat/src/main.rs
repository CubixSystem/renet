use client::ChatApp;
use eframe::{egui, epi};
use log::Level;
use renet_udp::renet::{channel::reliable::ReliableChannelConfig, error::DisconnectionReason};
use serde::{Deserialize, Serialize};

use history_logger::{HistoryLogger, LoggerApp};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

mod client;
mod history_logger;
mod server;

fn reliable_channels_config() -> Vec<ReliableChannelConfig> {
    let reliable_config = ReliableChannelConfig {
        message_resend_time: Duration::from_millis(500),
        ..Default::default()
    };
    vec![reliable_config]
}

#[derive(Debug, Serialize, Deserialize)]
enum ClientMessages {
    Text(String),
    Init { nick: String },
}

#[derive(Debug, Serialize, Deserialize)]
enum ServerMessages {
    ClientConnected(SocketAddr, String),
    ClientDisconnected(SocketAddr, DisconnectionReason),
    ClientMessage(SocketAddr, String),
    InitClient {
        clients: HashMap<SocketAddr, String>,
    },
}

fn main() {
    let app = App::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

#[derive(Debug, PartialEq, Eq)]
enum AppType {
    ChatApp,
    LoggerApp,
}

// Application to choose between chat or log.
struct App {
    application: AppType,
    chat_app: ChatApp,
    logger_app: LoggerApp,
}

impl App {
    pub fn new() -> Self {
        let logger = HistoryLogger::new(100, Level::Trace);
        let records = logger.records();
        logger.init();

        let logger_app = LoggerApp::new(records);
        let chat_app = ChatApp::default();

        Self {
            application: AppType::ChatApp,
            chat_app,
            logger_app,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "Renet Chat"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.application == AppType::ChatApp, "Chat App")
                    .clicked()
                {
                    self.application = AppType::ChatApp;
                }

                if ui
                    .selectable_label(self.application == AppType::LoggerApp, "Logger App")
                    .clicked()
                {
                    self.application = AppType::LoggerApp;
                }
            });
        });

        match self.application {
            AppType::ChatApp => self.chat_app.draw(ctx, frame),
            AppType::LoggerApp => self.logger_app.draw(ctx, frame),
        }

        self.chat_app.update();
        ctx.request_repaint();
    }
}
