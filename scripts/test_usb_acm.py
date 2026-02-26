import time
import glob
import sys

try:
    import serial
except ImportError:
    print("Error: 'pyserial' not found. Please install it using:")
    print("  python3 -m pip install pyserial")
    sys.exit(1)

def find_serial_port():
    # On macOS, serial ports for USB CDC ACM typically look like this
    # We look for the one with our serial number '12345678' first
    patterns = ['/dev/tty.usbmodem12345678*', '/dev/cu.usbmodem12345678*']
    for pattern in patterns:
        ports = glob.glob(pattern)
        if ports:
            return ports[0]
            
    # Fallback to any usbmodem
    patterns = ['/dev/tty.usbmodem*', '/dev/cu.usbmodem*']
    for pattern in patterns:
        ports = glob.glob(pattern)
        if ports:
            return ports[0]
    return None

def run_once():
    port = find_serial_port()
    if not port:
        return False
        
    print(f"Found port: {port}")
    try:
        # Open port
        ser = serial.Serial(port, baudrate=115200, timeout=2)
        ser.dtr = True
        ser.rts = True
        print(f"Opened {port} with DTR/RTS High")
        
        time.sleep(1)
        ser.reset_input_buffer()

        # Send HELLO
        msg = b"HELLO"
        success = False
        for i in range(3):
            print(f"Sending (attempt {i+1}): {msg}")
            ser.write(msg)
            response = ser.read(len(msg))
            print(f"Received: {response}")
            
            if response == msg:
                print("Echo verified!")
                success = True
                break
            time.sleep(0.5)
        
        if success:
            print("Sending QUIT...")
            ser.write(b"QUIT")
            time.sleep(0.5)
        
        ser.close()
        print("Waiting for next reset...")
        return True
        
    except Exception as e:
        print(f"Connection lost or error: {e}")
        return False

def main():
    print("USB Test Harness started. Press Ctrl+C to stop.")
    while True:
        if not run_once():
            time.sleep(0.5)

if __name__ == "__main__":
    main()

if __name__ == "__main__":
    main()
