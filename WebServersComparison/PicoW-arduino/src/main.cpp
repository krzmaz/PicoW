// based on example from https://docs.arduino.cc/library-examples/wifi-library/WiFiWebServer
// and https://github.com/earlephilhower/arduino-pico/blob/master/libraries/WiFi/examples/WiFiServer/WiFiServer.ino
#include <Arduino.h>
#include <WiFi.h>
#include <pico/cyw43_arch.h>

#ifndef STASSID
#define STASSID "ssid"
#define STAPSK "psk"
#endif

const char* ssid = STASSID;
const char* password = STAPSK;


int status = WL_IDLE_STATUS;
WiFiServer __not_in_flash("my_group_name") server(80);

// desperate attempt to make it faster by using __time_critical_func
static auto __not_in_flash("my_group_name") html = R"""(<!DOCTYPE html>
<html>
  <head> <title>Pico W</title> </head>
  <body> <h1>Pico W</h1>
    <p>Hello World</p>
  </body>
</html>
)""";

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
  pinMode(LED_BUILTIN, OUTPUT);
  WiFi.mode(WIFI_STA);
  WiFi.setHostname("PicoW");
  Serial.printf("Connecting to '%s' with '%s'\n", ssid, password);
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    Serial.print(".");
    delay(100);
  }
  digitalWrite(LED_BUILTIN, HIGH);
  Serial.printf("\nConnected to WiFi\n\nConnect to server at %s:%d\n", WiFi.localIP().toString().c_str(), 80);

  server.begin();
  // you're connected now, so print out the status:
  printWifiStatus();
}

void loop() {
  // listen for incoming clients
  WiFiClient client = server.available();
  if (client) {
    // an http request ends with a blank line
    bool currentLineIsBlank = true;
    while (client.connected()) {
      if (client.available()) {
        char c = client.read();
        // Serial.write(c);
        // if you've gotten to the end of the line (received a newline
        // character) and the line is blank, the http request has ended,
        // so you can send a reply
        if (c == '\n' && currentLineIsBlank) {
          // send a standard http response header
          client.println("HTTP/1.0 200 OK\r\nContent-type: text/html\r\n\r\n");
          client.printf(html);

          client.flush();
          break;
        }
        if (c == '\n') {
          // you're starting a new line
          currentLineIsBlank = true;
        } else if (c != '\r') {
          // you've gotten a character on the current line
          currentLineIsBlank = false;
        }
      }
    }
    // give the web browser time to receive the data
    delay(1);
    // close the connection:
    client.stop();
  }
}
