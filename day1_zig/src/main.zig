const std = @import("std");
const print = std.debug.print;
const ReduceOp = std.builtin.ReduceOp;

fn readFileToString(allocator: *std.mem.Allocator, path: []const u8) []u8 {
    const file = std.fs.cwd().openFile(path, .{}) catch |err| {
        std.debug.print("Failed to open file: {}\n", .{err});
        return &[_]u8{};
    };
    defer file.close();

    const fileSize = file.getEndPos() catch |err| {
        std.debug.print("Failed to get file size: {}\n", .{err});
        return &[_]u8{};
    };

    const buffer = allocator.alloc(u8, fileSize) catch |err| {
        std.debug.print("Failed to allocate buffer: {}\n", .{err});
        return &[_]u8{};
    };

    _ = file.read(buffer) catch |err| {
        std.debug.print("Failed to read file: {}\n", .{err});
        return &[_]u8{};
    };

    return buffer;
}

fn processFirst() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);

    var allocator = gpa.allocator();

    const input = readFileToString(&allocator, "input1.txt");
    defer allocator.free(input);

    var left = std.ArrayList(i32).init(std.heap.page_allocator);
    defer left.deinit();

    var right = std.ArrayList(i32).init(std.heap.page_allocator);
    defer right.deinit();

    var tokenizer = std.mem.tokenizeAny(u8, input, "\n");
    while (tokenizer.next()) |line| {
        var items = std.mem.tokenizeAny(u8, line, "   ");
        const first = try std.fmt.parseInt(i32, items.next().?, 10);
        const second = try std.fmt.parseInt(i32, items.next().?, 10);
        try left.append(first);
        try right.append(second);
    }

    std.mem.sort(i32, left.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right.items, {}, comptime std.sort.asc(i32));

    var result: u64 = 0;
    for (left.items, 0..) |l, i| {
        result += @abs(l - right.items[i]);
    }

    std.debug.print("{d}\n", .{result});
}

fn processSecond() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer std.debug.assert(gpa.deinit() == .ok);

    var allocator = gpa.allocator();

    const input = readFileToString(&allocator, "input1.txt");
    defer allocator.free(input);

    var left = std.ArrayList(i32).init(std.heap.page_allocator);
    defer left.deinit();

    var right = std.ArrayList(i32).init(std.heap.page_allocator);
    defer right.deinit();

    var tokenizer = std.mem.tokenizeAny(u8, input, "\n");
    while (tokenizer.next()) |line| {
        var items = std.mem.tokenizeAny(u8, line, "   ");
        const first = try std.fmt.parseInt(i32, items.next().?, 10);
        const second = try std.fmt.parseInt(i32, items.next().?, 10);
        try left.append(first);
        try right.append(second);
    }

    var result: i32 = 0;

    for (left.items) |n| {
        var count: i32 = 0;

        for (right.items) |r| {
            if (r == n) {
                count += 1;
            }
        }

        result += n * count;
    }

    std.debug.print("{d}\n", .{result});
}

pub fn main() !void {
    try processFirst();
    try processSecond();
}
