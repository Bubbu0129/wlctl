@@
-                Row::new(vec![
-                    Line::from(net.name.clone()).centered(),
-                    Line::from(net.network_type.to_string()).centered(),
-                    Line::from({
-                        match signal_percent {
-                            n if n >= 75 => format!("{signal_percent:3}% 󰤨"),
-                            n if (50..75).contains(&n) => format!("{signal_percent:3}% 󰤥"),
-                            n if (25..50).contains(&n) => format!("{signal_percent:3}% 󰤢"),
-                            _ => format!("{signal_percent:3}% 󰤟"),
-                        }
-                    })
-                    .centered(),
-                ])
+                Row::new(vec![
+                    Line::from(net.name.clone()).centered(),
+                    Line::from(net.network_type.to_string()).centered(),
+                    Line::from({
+                        if config.ascii {
+                            format!("{signal_percent:3}%")
+                        } else {
+                            match signal_percent {
+                                n if n >= 75 => format!("{signal_percent:3}% 󰤨"),
+                                n if (50..75).contains(&n) => format!("{signal_percent:3}% 󰤥"),
+                                n if (25..50).contains(&n) => format!("{signal_percent:3}% 󰤢"),
+                                _ => format!("{signal_percent:3}% 󰤟"),
+                            }
+                        }
+                    })
+                    .centered(),
+                ])
@@
-                if focused_block == FocusedBlock::NewNetworks {
+                if focused_block == FocusedBlock::NewNetworks {
@@
-                spans.extend(hint("k,", "  Up"));
-                spans.extend(hint("j,", "  Down"));
-                spans.extend(hint("󱁐  or ↵ ", " Connect"));
+                spans.extend(hint("k,", "  Up"));
+                spans.extend(hint("j,", "  Down"));
+                spans.extend(hint(if config.ascii { "Enter" } else { "󱁐  or ↵ " }, " Connect"));
@@
-                    bottom.extend(hint("k,", "  Up"));
-                    bottom.extend(hint("j,", "  Down"));
-                    bottom.extend(hint("ctrl+r", " Switch Mode"));
-                    bottom.extend(hint("⇄", " Nav"));
+                    bottom.extend(hint("k,", "  Up"));
+                    bottom.extend(hint("j,", "  Down"));
+                    bottom.extend(hint("ctrl+r", " Switch Mode"));
+                    bottom.extend(hint(if config.ascii { "Nav" } else { "⇄" }, " Nav"));
@@
-            FocusedBlock::AdapterInfos => {
-                vec![Line::from(vec![
-                    Span::from("󱊷 ").bold(),
-                    Span::from(" Discard"),
-                ])]
-            }
+            FocusedBlock::AdapterInfos => {
+                if config.ascii {
+                    vec![Line::from(vec![Span::from("Discard").bold()])]
+                } else {
+                    vec![Line::from(vec![Span::from("󱊷 ").bold(), Span::from(" Discard")])]
+                }
+            }
@@
-            FocusedBlock::PskAuthKey => vec![Line::from(vec![
-                Span::from(" ↵ ").bold(),
-                Span::from(" Apply"),
-                Span::from(" | "),
-                Span::from("⇄").bold(),
-                Span::from(" Hide/Show password"),
-                Span::from(" | "),
-                Span::from("󱊷 ").bold(),
-                Span::from(" Discard"),
-            ])],
+            FocusedBlock::PskAuthKey => vec![Line::from(vec![
+                Span::from(" ↵ ").bold(),
+                Span::from(" Apply"),
+                Span::from(" | "),
+                Span::from(if config.ascii { "Nav" } else { "⇄" }).bold(),
+                Span::from(" Hide/Show password"),
+                Span::from(" | "),
+                Span::from(if config.ascii { "Discard" } else { "󱊷 " }).bold(),
+                Span::from(if config.ascii { "" } else { " Discard" }),
+            ])],
@@
-            _ => vec![Line::from(vec![
-                Span::from("󱊷 ").bold(),
-                Span::from(" Discard"),
-            ])],
+            _ => {
+                if config.ascii {
+                    vec![Line::from(vec![Span::from("Discard").bold()])]
+                } else {
+                    vec![Line::from(vec![Span::from("󱊷 ").bold(), Span::from(" Discard")])]
+                }
+            }
         };
*** End Patch