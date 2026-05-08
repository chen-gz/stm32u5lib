const std = @import("std");

pub fn main() !void {
    const port_path = "/dev/ttyACM0"; // Default for Linux USB ACM

    std.debug.print("Opening serial port: {s}\n", .{port_path});

    var file = std.fs.openFileAbsolute(port_path, .{ .mode = .read_write }) catch |err| {
        std.debug.print("Failed to open port: {any}\n", .{err});
        std.debug.print("Make sure the device is connected and you have permissions (e.g., dialout group).\n", .{});
        return err;
    };
    defer file.close();

    // Send HELLO
    const msg = "HELLO";
    std.debug.print("Sending: {s}\n", .{msg});
    try file.writeAll(msg);

    // Read response
    var buffer: [10]u8 = undefined;
    const bytes_read = try file.read(&buffer);

    std.debug.print("Received: {s}\n", .{buffer[0..bytes_read]});

    if (std.mem.eql(u8, buffer[0..bytes_read], msg)) {
        std.debug.print("Echo verified!\n", .{});
    } else {
        std.debug.print("Echo failed!\n", .{});
    }
}
