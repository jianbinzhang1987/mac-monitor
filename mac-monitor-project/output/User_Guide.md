# Mac Monitor - User Guide

## 1. Installation & Auto-Start
Ensure you have administrative privileges on your macOS device.
- **Auto-Start**: The application is configured to automatically launch at login. This is handled by the system's Launch Agents.
- **Auto-Proxy**: Upon application launch, the HTTP Proxy mode is automatically enabled after a short delay (2 seconds). You may be prompted for your password once per session to authorize the helper tool.

## 2. Using HTTP Proxy Mode
This mode allows traffic auditing without installing a Network Extension (VPN profile).

### 2.1 Enable Proxy
1. Open the Mac Monitor GUI.
2. Navigate to the Proxy/Audit section (or use the command line helper).
3. Click **Enable Proxy**.
4. Enter your system password when prompted (required to trust the audit certificate and set system proxy).

> **Note**: If you are using the command line:
> ```bash
> sudo ./vpn-helper-x86_64-apple-darwin --enable-proxy
> ```

### 2.2 First Time Setup (Certificate Trust)
The system automatically adds the "Mac Monitor Root CA" to your keychain.
**Important**: If you see "Your connection is not private" in your browser:
1. Restart your browser (Cmd+Q).
2. If the issue persists, execute the cleanup command:
   ```bash
   sudo security delete-certificate -c "Mac Monitor Root CA" /Library/Keychains/System.keychain
   ```
   Then re-enable the proxy.

### 2.3 Verify Operation
- Visit `https://www.google.com`. The connection should be secure, and the certificate issuer should be "Mac Monitor Root CA".
- Traffic logs will appear in the Audit Log section.

### 2.4 Disable Proxy
1. Click **Disable Proxy** in the GUI.
2. Or use command line:
   ```bash
   sudo ./vpn-helper-x86_64-apple-darwin --disable-proxy
   ```
This will restore your system network settings and stop the background proxy service.
