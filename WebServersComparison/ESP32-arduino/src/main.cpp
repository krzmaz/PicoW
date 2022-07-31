// based on example from https://docs.arduino.cc/library-examples/wifi-library/WiFiWebServer
// and https://github.com/earlephilhower/arduino-pico/blob/master/libraries/WiFi/examples/WiFiServer/WiFiServer.ino
#include "esp_attr.h"
#include <Arduino.h>
#include <WiFi.h>
#include <WebServer.h>

#ifndef STASSID
#define STASSID "ssid"
#define STAPSK "psk"
#endif

const char* ssid = STASSID;
const char* password = STAPSK;


int status = WL_IDLE_STATUS;
WebServer server(80);


IRAM_ATTR static auto html = R"""(<!DOCTYPE html>
<html>
  <head> <title>Pico W</title> </head>
  <body> <h1>Pico W</h1>
    <p>Hello World</p>
  </body>
</html>
)""";

IRAM_ATTR static void handle_OnConnect() {

  server.send(200, "text/html", String(html));
}

void handle_NotFound(){
  server.send(404, "text/plain", "Not found");
}

void printWifiStatus() {
  // print the SSID of the network you're attached to:
  Serial.print("SSID: ");
  Serial.println(WiFi.SSID());
  // print your WiFi shield's IP address:
  IPAddress ip = WiFi.localIP();
  Serial.print("IP Address: ");
  Serial.println(ip);
  // print the received signal strength:
  long rssi = WiFi.RSSI();
  Serial.print("signal strength (RSSI):");
  Serial.print(rssi);
  Serial.println(" dBm");
}
void setup() {
  Serial.begin(115200);
  Serial.print("SETUP");
  WiFi.mode(WIFI_STA);
  WiFi.setHostname("PicoW");
  Serial.printf("Connecting to '%s' with '%s'\n", ssid, password);
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    delay(100);
  }
  Serial.printf("\nConnected to WiFi\n\nConnect to server at %s:%d\n", WiFi.localIP().toString().c_str(), 80);

  server.on("/", handle_OnConnect);
  server.onNotFound(handle_NotFound);

  server.begin();
  Serial.println("HTTP server started");
  // you're connected now, so print out the status:
  printWifiStatus();
}

IRAM_ATTR void loop() {
  server.handleClient();
}
