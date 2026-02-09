# NotifyMe Skill

This skill allows agents to send "high impact" notifications to the user using the `notify-me` CLI tool.
It is designed for critical alerts where the agent needs to grab the user's attention (e.g., blocking issues, manual intervention required).

## Installation

Ensure `notifyme` is installed and available in your PATH, or reference the binary path directly.

## Tools

### `notify_send`

Sends a high-priority notification via configured channels (Telegram, Lark, etc.).
The notification will attempt to mention the user and use loud settings where applicable.

**Command Structure:**
`notifyme send <headline> [--config-set <config_set>]`

**Description:**
Sends a notification. This is always treated as a high-impact event requiring user attention.

**Parameters:**

- `headline` (string, required): The short summary of the event (max 100 chars recommended).
- `config_set` (string, optional, default "default"): The configuration set to use (e.g., "default", "work", "personal").

**Usage Examples:**

1.  **Critical issue (Intervention needed)**:
    ```bash
    notifyme send "Database migration failed. Manual rollback required."
    ```

2.  **Blocking progress**:
    ```bash
    notifyme send "Cannot proceed without API key. Please check configuration."
    ```