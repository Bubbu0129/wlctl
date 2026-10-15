use std::sync::atomic::Ordering;

use crate::nm::Mode;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Clear, Paragraph},
};

use crate::app::{AdapterView, App, FocusedBlock};

pub fn render(app: &mut App, frame: &mut Frame) {
    if app.reset.enable {
        app.reset.render(frame);
    } else {
        let view = AdapterView {
            adapters: &app.adapters,
            active_index: app.active_index,
            selection_index: app.adapter_selection_index,
        };

        if !app.device.is_powered {
            app.device.render(
                frame,
                app.focused_block,
                app.config.clone(),
                &view,
                app.ethernet.as_ref(),
            )
        } else {
            let device = app.device.clone();
            match app.device.mode {
                Mode::Station => {
                    let primary = app.primary_link.clone();
                    if let Some(station) = &mut app.device.station {
                        station.render(
                            frame,
                            app.focused_block,
                            &device,
                            app.config.clone(),
                            &view,
                            primary.as_ref(),
                        );
                    }
                }
                Mode::Ap => {
                    if let Some(ap) = &mut app.device.ap {
                        ap.render(frame, app.focused_block, &device, app.config.clone(), &view);
                    }
                }
            }
        };

        if app.focused_block == FocusedBlock::WpaEntrepriseAuth
            && let Some(eap) = &mut app.auth.eap
        {
            eap.render(frame);
        }

        if app.focused_block == FocusedBlock::AdapterInfos {
            app.adapter.render(frame, app.device.address.clone());
        }

        if app.focused_block == FocusedBlock::Doctor
            && let Some(modal) = &app.doctor
        {
            crate::doctor::render_modal(frame, modal);
        }

        if app.focused_block == FocusedBlock::Vpn
            && let Some(modal) = &app.vpn
        {
            crate::vpn::render_modal(frame, modal);
        }

        if app.focused_block == FocusedBlock::HiddenSsidInput {
            app.auth.hidden.render(frame, app.config.ascii);
        }

        if app.agent.psk_required.load(Ordering::Relaxed) {
            app.focused_block = FocusedBlock::PskAuthKey;

            app.auth
                .psk
                .render(frame, app.network_name_requiring_auth.clone(), app.config.ascii);
        }

        if app
            .agent
            .private_key_passphrase_required
            .load(Ordering::Relaxed)
            && let Some(req) = &app.auth.request_key_passphrase
        {
            req.render(frame);
        }

        if app.agent.password_required.load(Ordering::Relaxed)
            && let Some(req) = &app.auth.request_password
        {
            req.render(frame);
        }

        if app
            .agent
            .username_and_password_required
            .load(Ordering::Relaxed)
            && let Some(req) = &app.auth.request_username_and_password
        {
            req.render(frame);
        }

        render_vpn_badge(frame, &app.active_vpns);

        // Notifications
        for (index, notification) in app.notifications.iter().enumerate() {
            notification.render(index, frame);
        }
    }
}
