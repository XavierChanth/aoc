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

    const robots = try read_data(alloc);
    defer alloc.free(robots);

    const quadrants = sum_quadrants(robots);
    const prod = product(&quadrants);

    const res = try std.fmt.allocPrint(alloc, "Result: {d}\n", .{prod});
    defer alloc.free(res);
    _ = try stdout.write(res);
}

fn sum_quadrants(robots: []const Robot) [4]u32 {
    var quadrants = [_]u32{ 0, 0, 0, 0 };
    for (robots) |robot| {
        var quad: u2 = 0; // 00
        const x = @mod(t * robot.xv + robot.xp, width);
        const y = @mod(t * robot.yv + robot.yp, height);
        if (x == x_mid or y == y_mid) continue;
        if (x > x_mid) quad += 1;
        if (y > y_mid) quad += 2;
        quadrants[quad] += 1;
    }
    return quadrants;
}

test "sum_quadrants" {
    const robots = [_]Robot{
        Robot{
            .xp = 0,
            .yp = 0,
            .xv = 0,
            .yv = 0,
        },
        Robot{
            .xp = 0,
            .yp = 0,
            .xv = 0,
            .yv = 1,
        },
        Robot{
            .xp = 0,
            .yp = 0,
            .xv = 1,
            .yv = 0,
        },
        Robot{
            .xp = 0,
            .yp = 0,
            .xv = 1,
            .yv = 1,
        },
    };
    const quadrants = sum_quadrants(&robots);

    try expect(quadrants[0] == 1);
    try expect(quadrants[1] == 1);
    try expect(quadrants[2] == 1);
    try expect(quadrants[3] == 1);
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
    xp: i16,
    yp: i16,
    xv: i16,
    yv: i16,
};

fn read_data(alloc: std.mem.Allocator) ![]Robot {
    const bytes = (try read_file(alloc));
    defer alloc.free(bytes);

    var lines = std.mem.splitScalar(u8, bytes, '\n');
    var robots = std.ArrayList(Robot).init(alloc);

    while (lines.next()) |line| {
        if (line.len == 0) continue;
        try robots.append(try parse_line(line));
    }

    return robots.toOwnedSlice();
}

test "read_data" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    const robots = try read_data(alloc);
    defer {
        alloc.free(robots);
        _ = gpa.deinit();
    }

    const robot = robots[0];

    try expect(robot.xp == 39);
    try expect(robot.yp == 28);
    try expect(robot.xv == 73);
    try expect(robot.yv == -88);
}

fn read_file(alloc: std.mem.Allocator) ![]u8 {
    const bytes = try std.fs.cwd().readFileAlloc(alloc, input_file_name, 100_000);
    return bytes;
}

test "read_file" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();
    defer _ = gpa.deinit();

    const bytes = (try read_file(alloc));
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
        .xp = xi,
        .yp = yi,
        .xv = xv,
        .yv = yv,
    };
}

test "parse_line" {
    const line = "p=39,28 v=73,-88";
    const robot = try parse_line(line);

    try expect(robot.xp == 39);
    try expect(robot.yp == 28);
    try expect(robot.xv == 73);
    try expect(robot.yv == -88);
}
