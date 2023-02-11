#include <WiFi.h>
#include <HTTPClient.h>
#include <ArduinoJson.h>
#define SSID_NAME "iPhone"
#define SSID_PASS "12345678"

#define LED_BUILTIN 2
void setup()
{
    pinMode(LED_BUILTIN, OUTPUT);
    Serial.begin(115200);
    WiFi.mode(WIFI_STA);
    WiFi.disconnect();
    delay(100);
    Serial.println();
    Serial.print("Connecting to ");
    Serial.println(SSID_NAME);
    WiFi.begin(SSID_NAME, SSID_PASS);
    while (WiFi.status() != WL_CONNECTED)
    {
        delay(500);
        Serial.print(".");
    }
    Serial.println("");
    Serial.println("WiFi connected");
    Serial.println("IP address: ");
    Serial.println(WiFi.localIP());
}
void loop()
{
    if (WiFi.status() == WL_CONNECTED)
    {
        HTTPClient http;
        http.begin("http://172.20.10.3:8082/datas/Sensor3");
        http.addHeader("Content-Type", "application/json");
        http.addHeader("Accept", "*/*");
        int httpResponseCode = http.GET();

        if (httpResponseCode == 200)
        {
            DynamicJsonDocument doc(2048);

            String payload = http.getString();
            deserializeJson(doc, payload);
            bool status = doc["status"];
            if (status == 1)
            {
                digitalWrite(LED_BUILTIN, HIGH);
            }
            else
            {
                digitalWrite(LED_BUILTIN, LOW);
            }
        }
        else
        {
            ESP.restart();
        }

        http.end(); // Close connection
    }
    delay(500);
}
