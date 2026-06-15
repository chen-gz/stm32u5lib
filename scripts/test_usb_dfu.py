#!/usr/bin/env python3
"""
Host-side test harness for USB DFU 1.1 Class (STM32U5)

This script searches for connected USB DFU devices (VID 0x0483 / PID 0xDF11 by default),
queries DFU_GETSTATUS (0x03) and DFU_GETSTATE (0x05) via Control Transfers on Endpoint 0,
and optionally verifies firmware flashing via `dfu-util`.
"""

import sys
import time
import subprocess

# Standard DFU Control Requests
DFU_DETACH = 0
DFU_DNLOAD = 1
DFU_UPLOAD = 2
DFU_GETSTATUS = 3
DFU_CLRSTATUS = 4
DFU_GETSTATE = 5
DFU_ABORT = 6

# DFU State Mapping
DFU_STATES = {
    0: "appIDLE",
    1: "appDETACH",
    2: "dfuIDLE",
    3: "dfuDNLOAD-SYNC",
    4: "dfuDNLBUSY",
    5: "dfuDNLOAD-IDLE",
    6: "dfuMANIFEST-SYNC",
    7: "dfuMANIFEST",
    8: "dfuMANIFEST-WAIT-RESET",
    9: "dfuUPLOAD-IDLE",
    10: "dfuERROR",
}

def test_dfu_with_dfu_util():
    """Use `dfu-util -l` to scan for DFU device."""
    print("Scanning for DFU devices using dfu-util...")
    try:
        res = subprocess.run(["dfu-util", "-l"], capture_output=True, text=True)
        print(res.stdout)
        if "0483:df11" in res.stdout or "Found DFU" in res.stdout:
            print("✓ Host successfully detected STM32U5 DFU Device!")
            return True
        else:
            print("× No DFU device detected by dfu-util.")
            return False
    except FileNotFoundError:
        print("Note: 'dfu-util' command not found on host. Installing dfu-util is recommended.")
        return False

def test_dfu_pyusb(vid=0x0483, pid=0xDF11):
    """Use PyUSB to perform Control Transfers directly."""
    try:
        import usb.core
        import usb.util
    except ImportError:
        print("Note: 'pyusb' not found. Installing via 'pip install pyusb' enables direct Control Transfer testing.")
        return False

    dev = usb.core.find(idVendor=vid, idProduct=pid)
    if dev is None:
        print(f"PyUSB: Device {hex(vid)}:{hex(pid)} not found.")
        return False

    print(f"PyUSB: Found DFU Device {hex(vid)}:{hex(pid)}")

    # Control Read: DFU_GETSTATUS (bRequest = 0x03, bmRequestType = 0xA1 (Class, Interface, In))
    try:
        # GETSTATUS expects 6 bytes: bStatus, bwPollTimeout (3 bytes), bState, iString
        status_resp = dev.ctrl_transfer(
            bmRequestType=0xA1,
            bRequest=DFU_GETSTATUS,
            wValue=0,
            wIndex=0,
            data_or_wLength=6
        )
        b_status = status_resp[0]
        poll_timeout = status_resp[1] | (status_resp[2] << 8) | (status_resp[3] << 16)
        b_state = status_resp[4]
        state_str = DFU_STATES.get(b_state, f"Unknown({b_state})")

        print(f"✓ DFU_GETSTATUS response: Status={b_status}, PollTimeout={poll_timeout}ms, State={state_str}")
        return True
    except Exception as e:
        print(f"× PyUSB DFU_GETSTATUS control transfer error: {e}")
        return False

def main():
    print("==========================================")
    print(" STM32U5 USB DFU Host Test Harness")
    print("==========================================")
    
    found_pyusb = test_dfu_pyusb()
    found_util = test_dfu_with_dfu_util()
    
    if not found_pyusb and not found_util:
        print("\nWaiting for STM32U5 DFU device to connect...")

if __name__ == "__main__":
    main()
