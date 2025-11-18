# `GooglePlay.proto`

This is sourced from [AuroraOSS](https://gitlab.com/AuroraOSS/gplayapi/-/blob/master/lib/src/main/proto/GooglePlay.proto), with the following patch applied:

```
@@ -23,6 +23,7 @@
   optional int64 type = 17;
   optional CompressedAppData compressedAppData = 18;
   optional string sha256 = 19;
+  optional DexMetadata dexMetadata = 21;
 }
 
 message SplitDeliveryData {
@@ -51,6 +52,12 @@
   optional string downloadUrl = 3;
 }
 
+message DexMetadata {
+  optional int64 downloadSize = 1;
+  optional string sha256 = 2;
+  optional string downloadUrl = 3;
+}
+
 message AppFileMetadata {
   optional int32 fileType = 1;
   optional int32 versionCode = 2;
```
