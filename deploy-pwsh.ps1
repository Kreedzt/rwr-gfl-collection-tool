cp .\target\release\rwr-gfl-mod-collection-tool.exe deploy\
cp .\LICENSE deploy\
windeployqt.exe .\deploy\rwr-gfl-mod-collection-tool.exe --dir deploy --release --no-translations --no-angle --no-opengl-sw --no-quick-import --no-virtualkeyboard --no-compiler-runtime --no-webkit2
7z.exe a deploy.7z .\deploy\*
