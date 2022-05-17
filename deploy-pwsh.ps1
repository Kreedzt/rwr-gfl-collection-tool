cp .\target\release\rwr-collection-tool.exe deploy\
cp .\LICENSE deploy\
windeployqt.exe .\deploy\rwr-collection-tool.exe --dir deploy --release --no-translations --no-angle --no-opengl-sw --no-quick-import --no-virtualkeyboard --no-compiler-runtime --no-webkit2
7z.exe a deploy.7z .\deploy\*
