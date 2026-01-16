import socket
import json
import os
import sys

def test_ipc_registration():
    socket_path = "/tmp/mac_monitor_audit.sock"

    if not os.path.exists(socket_path):
        print(f"❌ Socket not found at {socket_path}. Is the Audit Service running?")
        return

    try:
        # 1. Create a Unix Domain Socket
        client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        client.connect(socket_path)

        # 2. Prepare Registration Command
        reg_command = {
            "command": "register",
            "payload": {
                "server_ip": "127.0.0.1",
                "server_port": "8080",
                "cpe_id": "TEST_DEVICE_001",
                "pin": "666888"
            }
        }

        # 3. Send Command
        print(f"📤 Sending registration command: {json.dumps(reg_command)}")
        client.sendall(json.dumps(reg_command).encode('utf-8'))

        # Shutdown sending to signal EOF if the other side uses read_to_string (though we fixed it to read chunk)
        client.shutdown(socket.SHUT_WR)

        # 4. Receive Response
        response_data = b""
        while True:
            chunk = client.recv(4096)
            if not chunk:
                break
            response_data += chunk

        if response_data:
            response = json.loads(response_data.decode('utf-8'))
            print(f"📥 Received response: {json.dumps(response, indent=2)}")
            if response.get("status") == "ok":
                print("✅ IPC Registration Test PASSED")
            else:
                print(f"❌ IPC Registration Test FAILED: {response.get('message')}")
        else:
            print("❌ IPC Registration Test FAILED: No response received")

        client.close()

    except Exception as e:
        print(f"❌ Error during IPC test: {e}")

if __name__ == "__main__":
    test_ipc_registration()
