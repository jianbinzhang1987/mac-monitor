
import socket
import json
import sqlite3
import time
import os
import uuid

SOCKET_PATH = "/tmp/mac_monitor_audit.sock"
DB_PATH = "/Users/adolf/Desktop/mac-monitor/db/audit.db"

def send_ipc_command(command, payload):
    if not os.path.exists(SOCKET_PATH):
        print(f"❌ Socket not found at {SOCKET_PATH}")
        return False

    try:
        client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        client.connect(SOCKET_PATH)
        
        msg = {
            "command": command,
            "payload": payload
        }
        
        client.sendall(json.dumps(msg).encode('utf-8'))
        response = client.recv(4096)
        client.close()
        
        print(f"Sent: {command}, Response: {response.decode('utf-8')}")
        return True
    except Exception as e:
        print(f"❌ IPC Error: {e}")
        return False

def check_db_for_domain(domain):
    try:
        conn = sqlite3.connect(DB_PATH)
        cursor = conn.cursor()
        cursor.execute("SELECT count(*) FROM monitor_log_traffic WHERE domain = ?", (domain,))
        count = cursor.fetchone()[0]
        conn.close()
        return count
    except Exception as e:
        print(f"❌ DB Error: {e}")
        return -1

def run_test():
    print("--- Starting Verification ---")
    
    # Baseline counts
    initial_github_count = check_db_for_domain("github.com")
    initial_google_count = check_db_for_domain("www.google.com")
    initial_other_count = check_db_for_domain("example.com")
    
    print(f"Baseline: github={initial_github_count}, google={initial_google_count}, other={initial_other_count}")

    # Test Case 1: Allowed (github.com)
    log_github = {
        "id": str(uuid.uuid4()),
        "cpe_id": "test_cpe",
        "url": "https://github.com/test",
        "req_time": "2023-10-27 10:00:00",
        "method_type": "GET",
        "domain": "github.com",
        "process_name": "test_proc",
        "risk_level": 0,
        "ip": "127.0.0.1",
        "mac": "00:00:00:00:00:00",
        "host_id": "test_host"
    }
    send_ipc_command("log_traffic", log_github)

    # Test Case 2: Allowed (google.com)
    log_google = {
        "id": str(uuid.uuid4()),
        "cpe_id": "test_cpe",
        "url": "https://www.google.com/search",
        "req_time": "2023-10-27 10:01:00",
        "method_type": "GET",
        "domain": "www.google.com",
        "process_name": "test_proc",
        "risk_level": 0,
        "ip": "127.0.0.1",
        "mac": "00:00:00:00:00:00",
        "host_id": "test_host"
    }
    send_ipc_command("log_traffic", log_google)
    
    # Test Case 3: Blocked (example.com)
    log_other = {
        "id": str(uuid.uuid4()),
        "cpe_id": "test_cpe",
        "url": "https://example.com",
        "req_time": "2023-10-27 10:02:00",
        "method_type": "GET",
        "domain": "example.com",
        "process_name": "test_proc",
        "risk_level": 0,
        "ip": "127.0.0.1",
        "mac": "00:00:00:00:00:00",
        "host_id": "test_host"
    }
    send_ipc_command("log_traffic", log_other)

    time.sleep(2) # Wait for async wite

    # Final counts
    final_github_count = check_db_for_domain("github.com")
    final_google_count = check_db_for_domain("www.google.com")
    final_other_count = check_db_for_domain("example.com")
    
    print(f"Final: github={final_github_count}, google={final_google_count}, other={final_other_count}")

    success = True
    if final_github_count != initial_github_count + 1:
        print("❌ Failed: github.com log not saved")
        success = False
    
    if final_google_count != initial_google_count + 1:
        print("❌ Failed: www.google.com log not saved")
        success = False
        
    if final_other_count != initial_other_count:
        print("❌ Failed: example.com log WAS saved (should be ignored)")
        success = False
        
    if success:
        print("✅ Verification Passed!")
    else:
        print("❌ Verification Failed")

if __name__ == "__main__":
    run_test()
