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
    patterns = ['/dev/tty.usbmodem*', '/dev/cu.usbmodem*']
    for pattern in patterns:
        ports = glob.glob(pattern)
        if ports:
            return ports[0]
    return None

def main():
    print("Waiting for USB serial port...")
    port = None
    timeout = 30
    start_time = time.time()
    
    while time.time() - start_time < timeout:
        port = find_serial_port()
        if port:
            break
        time.sleep(0.5)
        
    if not port:
        print("Error: USB serial port not found.")
        sys.exit(1)
        
    print(f"Found port: {port}")
    
    try:
        # Open port
        port = "/dev/tty.usbmodem123456781"
        ser = serial.Serial(port, baudrate=115200, timeout=2)
        # Some systems need DTR to be set to trigger wait_connection() in embassy
        ser.dtr = True
        ser.rts = True
        print(f"Opened {port} with DTR/RTS High")
        
        # Give it a moment to initialize after DTR
        time.sleep(2)
        
        # Flush any garbage
        ser.reset_input_buffer()

        # Send HELLO
        msg = b"HELLO"
        for i in range(3): # Try up to 3 times
            print(f"Sending (attempt {i+1}): {msg}")
            ser.write(msg)
            
            # Read response
            response = ser.read(len(msg))
            print(f"Received: {response}")
            
            if response == msg:
                print("Echo verified!")
                break
            time.sleep(1)
        else:
            print(f"Error: Expected {msg}, got {response}")
            sys.exit(1)
        
        # Send QUIT
        print("Sending QUIT...")
        ser.write(b"QUIT")
        
        ser.close()
        print("Done!")
        sys.exit(0)
        
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
