# NOTIFY ME
> **Note**: This repository is currently under development. The documentation above represents the planned design and functionality. The installation methods and some commands may not be available yet.

A CLI tool to monitor long-running commands and send notifications in different ways including telegram, email, sms, phone call... 
You can customize your various config-set in seperate config files.

## Quick Start
1. Install the package:
linux:
```bash
sudo apt-get install notifyme
```
macOS:
```bash
brew install notifyme
```
2. [optional] Create a config file:
...

3. Run the command:
```bash
# Using the delimiter
notifyme run --config myconfig -- ping -c 5 google.com

# Default config
notifyme run -- ping -c 5 google.com
```

## command list

- run command
```bash
notifyme run [-c config-set] <command> <args>
```

- list config sets
```bash
notifyme list
```

- create config set
```bash
notifyme create <config set name>
```

- delete config set
```bash
notifyme delete <config set name>
```

- test config set
```bash
notifyme test <config set name>
```


## Configuration Format
configuration files stored in `~/.config/notifyme/configs/`
default config set name is `default`

```xml
<config-set name="default">
    <notification-configs>
    <config type="email">
      <to>email@example.com</to>
      <from>email@example.com</from>
      <subject>Test Email</subject>
      <body>This is a test email.</body>
      <smtp>
        <host>smtp.example.com</host>
        <port>587</port>
        <username>username</username>
        <password>password</password>
        <encryption>tls</encryption>
        <auth>true</auth>
        <debug>false</debug>
        <timeout>10</timeout>
        <tls_verify>false</tls_verify>
        <tls_ca_certs>/path/to/ca_certs</tls_ca_certs>
        <tls_key>/path/to/key</tls_key>
        <tls_cert>/path/to/cert</tls_cert>
        <tls_ciphers>TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256</tls_ciphers>

        </smtp>

    </config>

    <config type="telegram">
        <token>123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZ</token>
        <chat_id>123456789</chat_id>
        <message>This is a test message.</message>
        <parse_mode>HTML</parse_mode>
        <disable_web_page_preview>true</disable_web_page_preview>
        <disable_notification>false</disable_notification>
    </config>

    <config type="sms-twilio">
        <provider>twilio</provider>
        <account_sid>ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX</account_sid>
        <auth_token>your_auth_token</auth_token>
        <from>+15017122661</from>
        <to>+15558675310</to>
        <body>This is a test message.</body>
        <media_urls>
        <url>https://example.com/image.jpg</url>
        <url>https://example.com/image2.jpg</url>
        </media_urls>
        <mms>true</mms>
        <sender_id>ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX</sender_id>
        <carrier>att</carrier>
        <carrier_lookup>true</carrier_lookup>
        <carrier_lookup_country_code>US</carrier_lookup_country_code>
    </config>
    
    <config type="phone-call">
        <provider>twilio</provider>
        <account_sid>ACXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX</account_sid>
        <auth_token>your_auth_token</auth_token>
        <from>+15017122661</from>
        <to>+15558675310</to>
        <url>https://example.com/call.xml</url>
        <method>POST</method>
        <timeout>20</timeout>
        <record>true</record>
        <status_callback>https://example.com/status.php</status_callback>
        <status_callback_method>POST</status_callback_method>
        <machine_detection>true</machine_detection>
        <machine_detection_timeout>30</machine_detection_timeout>
        <machine_detection_url>https://example.com/machine_detection.php</machine_detection_url>
        <machine_detection_method>POST</machine_detection_method>
    </config>

    <config type="cmd">
        <command>ping</command>
        <args>-c 5 google.com</args>
        <timeout>10</timeout>
        <retry>3</retry>
        <retry_delay>5</retry_delay>
    </config>
    
    <config type="http">
        <url>https://example.com/api</url>
        <method>POST</method>
        <headers>
        <key>Content-Type</key>
        <value>application/json</value>
        </headers>
        <body>{"key": "value"}</body>
        <timeout>10</timeout>
        <retry>3</retry>
        <retry_delay>5</retry_delay>
    <config>
    </notification-configs>
</config-set>
```

# Next Steps
- Implement Notification Senders: Flesh out each notification sender module to perform actual API calls or actions.
- Handle Asynchronous Operations: Ensure that network operations are handled asynchronously using async/await and tokio.
- Enhance Error Handling: Provide detailed error messages and handle different error cases.
- Logging and Debugging: Use the log crate to add logging statements for better observability.
- Configuration Validation: Validate the configuration files and provide helpful feedback to users.