const std = @import("std");
const fs = std.fs;

const stdout = std.io.getStdOut();
const expect = std.testing.expect;

const input_file_name = "./day14.txt";

const t = 100;

const width = 101;
const height = 103;

const x_mid = width / 2;
const y_mid = height / 2;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const alloc = gpa.allocator();

    var robots = try read_data(alloc);
    defer robots.deinit();

    var quadrants = [_]u32{ 0, 0, 0, 0 };
    for (robots.items) |robot| {
        var quad: u2 = 0;
        const x = @mod(t * robot.xv + robot.xi, width);
        const y = @mod(t * robot.yv + robot.yi, height);
        if (x == x_mid or y == y_mid) continue;
        if (x > x_mid) quad += 1;
        if (y > y_mid) quad += 2;
        quadrants[quad] += 1;
    }
    const res = try std.fmt.allocPrint(alloc, "Result: {d}\n", .{product(&quadrants)});
    defer alloc.free(res);
    _ = try stdout.write(res);
}

fn product(data: []const u32) u32 {
    var res: u32 = 1;
    for (data) |x| {
        res *= x;
    }
    return res;
}

test "product" {
    try expect(product(&[_]u32{ 1, 2, 3 }) == 6);
    try expect(product(&[_]u32{ 1, 2, 2, 2 }) == 8);
}

const Robot = struct {
    xi: i16,
    yi: i16,
    xv: i16,
    yv: i16,
};

fn read_data(alloc: std.mem.Allocator) !std.ArrayList(Robot) {
    const bytes = (try read_file(alloc)).*;
    defer alloc.free(bytes);
    var lines = std.mem.split(u8, bytes, "\n");
    var robots = std.ArrayList(Robot).init(alloc);

    while (lines.next()) |line| {
        if (line.len == 0) continue;
        try robots.append(try parse_line(line));
    }

    return robots;
}

test "read_data" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    var robots = read_data(alloc) catch unreachable;
    defer {
        robots.deinit();
        _ = gpa.deinit();
    }

    const robot = robots.items[0];

    try expect(robot.xi == 39);
    try expect(robot.yi == 28);
    try expect(robot.xv == 73);
    try expect(robot.yv == -88);
}

fn read_file(alloc: std.mem.Allocator) !*const []u8 {
    const bytes = try std.fs.cwd().readFileAlloc(alloc, input_file_name, 100_000);
    return &bytes;
}

test "read_file" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    defer _ = gpa.deinit();
    const bytes = (try read_file(alloc)).*;
    defer alloc.free(bytes);
    try expect(bytes.len > 1000);
}

fn parse_line(line: []const u8) !Robot {
    const eq1 = std.mem.indexOfScalar(u8, line, '=').?;
    const cm1 = std.mem.indexOfScalarPos(u8, line, eq1, ',').?;
    const spc = std.mem.indexOfScalarPos(u8, line, cm1, ' ').?;
    const eq2 = std.mem.indexOfScalarPos(u8, line, spc, '=').?;
    const cm2 = std.mem.indexOfScalarPos(u8, line, eq2, ',').?;

    const xi = try std.fmt.parseInt(i16, line[eq1 + 1 .. cm1], 10);
    const yi = try std.fmt.parseInt(i16, line[cm1 + 1 .. spc], 10);
    const xv = try std.fmt.parseInt(i16, line[eq2 + 1 .. cm2], 10);
    const yv = try std.fmt.parseInt(i16, line[cm2 + 1 ..], 10);
    return Robot{
        .xi = xi,
        .yi = yi,
        .xv = xv,
        .yv = yv,
    };
}

test "parse_line" {
    const line = "p=39,28 v=73,-88";
    const robot = parse_line(line) catch unreachable;

    try expect(robot.xi == 39);
    try expect(robot.yi == 28);
    try expect(robot.xv == 73);
    try expect(robot.yv == -88);
}
