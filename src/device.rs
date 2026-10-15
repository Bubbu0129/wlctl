diff --git a/src/device.rs b/src/device.rs
index 0000000..0000000 100644
--- a/src/device.rs
+++ b/src/device.rs
@@
-pub fn adapter_nav_spans<'a>() -> Vec<Span<'a>> {
-    vec![
-        Span::from(" | "),
-        Span::from("j/k").bold(),
-        Span::from(" Move"),
-        Span::from(" | "),
-        Span::from("⏎").bold(),
-        Span::from(" Activate"),
-    ]
-}
+pub fn adapter_nav_spans<'a>(ascii: bool) -> Vec<Span<'a>> {
+    if ascii {
+        vec![
+            Span::from(" | "),
+            Span::from("j/k").bold(),
+            Span::from(" Move"),
+            Span::from(" | "),
+            Span::from("Enter").bold(),
+            Span::from(" Activate"),
+        ]
+    } else {
+        vec![
+            Span::from(" | "),
+            Span::from("j/k").bold(),
+            Span::from(" Move"),
+            Span::from(" | "),
+            Span::from("⏎").bold(),
+            Span::from(" Activate"),
+        ]
+    }
+}
