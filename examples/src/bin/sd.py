import os
import string
drives = ['%s:' % d for d in string.ascii_uppercase if os.path.exists('%s:' % d)]
print(drives)
import win32file
import win32con
# get physical drive list 
import wmi

for drive in wmi.WMI().Win32_DiskDrive():
    print(drive.Caption, drive.DeviceID, drive.Index, drive.InterfaceType, drive.Manufacturer, drive.MediaType, drive.Model, drive.Name, drive.PNPDeviceID, drive.SerialNumber, drive.Size, drive.SystemName, drive.TotalCylinders, drive.TotalHeads, drive.TotalSectors, drive.TotalTracks, drive.TracksPerCylinder)
# exit()
# drive = r"\\.\PhysicalDrive2"  # Replace with your SD card's device name
drive = r"\\.\PHYSICALDRIVE2"  # Replace with your SD card's device namek

def read_sector(drive, sector, sector_size=512):
    # Open the drive
    handle = win32file.CreateFile(
        drive,
        win32con.GENERIC_READ,
        win32con.FILE_SHARE_READ | win32con.FILE_SHARE_WRITE,
        None,
        win32con.OPEN_EXISTING,
        0,
        None
    )

    # Set the file pointer to the start of the desired sector
    win32file.SetFilePointer(handle, sector * sector_size, win32con.FILE_BEGIN)

    # Read the sector
    _, data = win32file.ReadFile(handle, sector_size)

    # Close the handle
    win32file.CloseHandle(handle)

    return data
def write_sector(drive, sector, data, sector_size=512):
    # Ensure the data length is equal to the sector size
    if len(data) != sector_size:
        raise ValueError("Data length must be equal to the sector size")

    # Open the drive
    handle = win32file.CreateFile(
        drive,
        win32con.GENERIC_WRITE,
        win32con.FILE_SHARE_READ | win32con.FILE_SHARE_WRITE,
        None,
        win32con.OPEN_EXISTING,
        0,
        None
    )

    # Set the file pointer to the start of the desired sector
    win32file.SetFilePointer(handle, sector * sector_size, win32con.FILE_BEGIN)

    # Write to the sector
    bytes_written, _ = win32file.WriteFile(handle, data)

    # Close the handle
    win32file.CloseHandle(handle)

    return bytes_written


def read_sectors(drive, start_sector, num_sectors, sector_size=512):
    # Open the drive
    handle = win32file.CreateFile(
        drive,
        win32con.GENERIC_READ,
        win32con.FILE_SHARE_READ | win32con.FILE_SHARE_WRITE,
        None,
        win32con.OPEN_EXISTING,
        0,
        None
    )

    # Set the file pointer to the start of the desired sector
    win32file.SetFilePointer(handle, start_sector * sector_size, win32con.FILE_BEGIN)

    # Read the sector
    _, data = win32file.ReadFile(handle, num_sectors * sector_size)

    # Close the handle
    win32file.CloseHandle(handle)

    return data

# Example usage
sector_data = read_sector(drive, 10000)  # Read the first sector
# print sector_data as hex
print(' '.join('{:02x}'.format(x) for x in sector_data))
# print(sector_data)

print()
data = b'\x00' * 512

# write_sector(drive, 11000, data)  # Write the first sector

sector_data = read_sector(drive, 11000)  # Read the first sector
print(' '.join('{:02x}'.format(x) for x in sector_data))


sector_data = read_sector(drive, 12000)  # Read the first sector
# print sector_data as hex
print(' '.join('{:02x}'.format(x) for x in sector_data))

pic_num: int = 11
multiplier: int = 1000
while True and pic_num < 100000:
    print("pic_num: ", pic_num)
    data = read_sectors(drive, pic_num * multiplier, multiplier)
    if data[0] != 0xff or data[1] != 0xd8:
        pic_num += 1 #
        continue
    # find the end of the picture, ff d9
    pic_end = 0
    for i in range(len(data)):
        if data[i] == 0xff and data[i+1] == 0xd9:
            pic_end = i + 1
            break
    # write the picture
    with open("./pic/" + str(pic_num) + ".jpg", "wb") as f:
        f.write(data[:pic_end])
    pic_num += 1 #