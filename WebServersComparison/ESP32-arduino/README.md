To improve performance we can put WebServer implementation into RAM:
```bash
# note the needed first argument on mac, for details see https://stackoverflow.com/a/525612
if [[ $OSTYPE == 'darwin'* ]]; then
    sed -i '' 's/void WebServer::/IRAM_ATTR void WebServer::/' ~/.platformio/packages/framework-arduinoespressif32/libraries/WebServer/src/WebServer.cpp
else
    sed -i 's/void WebServer::/IRAM_ATTR void WebServer::/' ~/.platformio/packages/framework-arduinoespressif32/libraries/WebServer/src/WebServer.cpp
fi
```