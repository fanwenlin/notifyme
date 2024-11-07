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
Configuration files are stored in `~/.config/notifyme/configs/`
The default config set name is `default`

```xml
<config-set name="default">
    <notification-configs>
        <email>
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
                <tls_ciphers>TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:...</tls_ciphers>
            </smtp>
        </email>

        <telegram>
            <token>YOUR_BOT_TOKEN</token>
            <chat_id>YOUR_CHAT_ID</chat_id>
            <message>This is a test message.</message>
            <parse_mode>HTML</parse_mode>
            <disable_web_page_preview>true</disable_web_page_preview>
            <disable_notification>false</disable_notification>
        </telegram>

        <lark>
            <webhook_url>https://open.feishu.cn/open-apis/bot/v2/hook/your-webhook-url</webhook_url>
            <sign_key>your-sign-key</sign_key>
            <at_user_id>optional-user-id-to-mention</at_user_id>
        </lark>

        <sms-twilio>
            <provider>twilio</provider>
            <account_sid>YOUR_ACCOUNT_SID</account_sid>
            <auth_token>YOUR_AUTH_TOKEN</auth_token>
            <from>+15017122661</from>
            <to>+15558675310</to>
            <body>This is a test message.</body>
            <media_urls>
                <url>https://example.com/image.jpg</url>
            </media_urls>
            <mms>true</mms>
            <sender_id>YOUR_SENDER_ID</sender_id>
            <carrier>att</carrier>
            <carrier_lookup>true</carrier_lookup>
            <carrier_lookup_country_code>US</carrier_lookup_country_code>
        </sms-twilio>
        
        <phone-call>
            <provider>twilio</provider>
            <account_sid>YOUR_ACCOUNT_SID</account_sid>
            <auth_token>YOUR_AUTH_TOKEN</auth_token>
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
        </phone-call>

        <cmd>
            <command>ping</command>
            <args>-c 5 google.com</args>
            <timeout>10</timeout>
            <retry>3</retry>
            <retry_delay>5</retry_delay>
        </cmd>
        
        <http>
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
        </http>
    </notification-configs>
</config-set>
```

# Next Steps
- Implement Notification Senders: Flesh out each notification sender module to perform actual API calls or actions.
- Handle Asynchronous Operations: Ensure that network operations are handled asynchronously using async/await and tokio.
- Enhance Error Handling: Provide detailed error messages and handle different error cases.
- Logging and Debugging: Use the log crate to add logging statements for better observability.
- Configuration Validation: Validate the configuration files and provide helpful feedback to users.