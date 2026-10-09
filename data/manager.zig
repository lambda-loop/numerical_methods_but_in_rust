// will provide all the matrixes infos in the data directory as table which will
// be then used by the project. use it from the root of the project

const std = @import("std");
const Io = std.Io;
const Dir = std.Io.Dir;
const File = std.Io.File;
const Allocator = std.mem.Allocator;

const as_path = "data/as/";
const out_file_name = "src/table";

// const config = @import("../src/config.zig");

pub const Matrix = struct {
    name: []const u8,
    len: usize,
    is_symmetric: bool,

    const Self = @This();
    pub fn print(
        self: Self,
    ) void {
        const sym_text = if (self.is_symmetric) "symmetric" else "assymetric";

        std.debug.print("Matrix[nome: {s}, tamanho: {}x{}, tipo: {s}]\n", .{ self.name, self.len, self.len, sym_text });
    }

    pub fn deinit(self: Self, allocator: Allocator) void {
        allocator.free(self.name);
    }
};

pub fn getMatrix(
    io: Io,
    dir: Dir,
    matrix_name: []const u8,
) !Matrix {
    var name_buf: [128]u8 = undefined;
    const file_name = try std.fmt.bufPrint(
        &name_buf,
        "{s}.mtx",
        .{matrix_name},
    );

    var buffer: [1024]u8 = undefined;
    const file_content = try dir.readFile(io, file_name, &buffer);
    var lines_iter = std.mem.splitScalar(u8, file_content, '\n');

    const fst_line = lines_iter.next().?;
    const is_symmetric = std.mem.find(u8, fst_line, "symmetric") != null;

    const snd_line = lines_iter.next().?;
    var sizes_iter = std.mem.splitAny(u8, snd_line, "\r\t ");

    const rows_str = sizes_iter.next().?;
    const cols_str = sizes_iter.next().?;

    const rows = try std.fmt.parseInt(usize, rows_str, 10);
    const cols = try std.fmt.parseInt(usize, cols_str, 10);

    std.debug.assert(rows == cols);
    return Matrix{
        .name = matrix_name,
        .len = rows,
        .is_symmetric = is_symmetric,
    };
}

pub fn getAllMatrixes(
    io: Io,
    allocator: Allocator,
    dir: Dir,
) ![]Matrix {
    var ms = std.ArrayList(Matrix).empty;

    var dir_iter = dir.iterate();
    while (try dir_iter.next(io)) |entry| {
        if (entry.kind != .file) continue;
        if (!std.mem.endsWith(u8, entry.name, ".mtx")) continue;

        const matrix_name = std.fs.path.stem(entry.name);
        const safe_name = try allocator.dupe(u8, matrix_name);

        const m = try getMatrix(io, dir, safe_name);
        try ms.append(allocator, m);
    }

    return ms.toOwnedSlice(allocator);
}

fn formatTable(allocator: Allocator, ms: []Matrix) ![]const u8 {
    var out = std.ArrayList([]const u8).empty;
    for (0..ms.len - 1) |i| {
        const m = ms[i];
        const m_str = try std.fmt.allocPrint(
            allocator,
            "{s}:{d}\n",
            .{ m.name, m.len },
        );

        try out.append(allocator, m_str);
    }
    const m = ms[ms.len - 1];
    const m_str = try std.fmt.allocPrint(
        allocator,
        "{s}:{d}",
        .{ m.name, m.len },
    );
    try out.append(allocator, m_str);
    const out_slice = try out.toOwnedSlice(allocator);
    defer {
        for (out_slice) |slice|
            allocator.free(slice);
        allocator.free(out_slice);
    }

    const t = std.mem.concat(allocator, u8, out_slice);
    return t;
}

pub fn main(init: std.process.Init) !void {
    const io = init.io;
    const gpa = init.gpa;
    // var ms = std.ArrayList(Matrix).empty;

    const as_dir = try std.Io.Dir.openDir(
        std.Io.Dir.cwd(),
        io,
        as_path,
        .{ .iterate = true },
    );
    defer as_dir.close(io);
    const ms = try getAllMatrixes(
        io,
        gpa,
        as_dir,
    );
    defer {
        for (ms) |m| {
            m.deinit(gpa);
        }
        gpa.free(ms);
    }

    const t = try formatTable(gpa, ms);
    defer gpa.free(t);

    const out_file = try Dir.cwd().createFile(io, out_file_name, .{
        .truncate = true,
        .exclusive = false,
    });
    defer out_file.close(io);

    var buffer: [1024]u8 = undefined;
    var writer = out_file.writer(io, &buffer);
    // var writer_interface = writer.interface;
    try writer.interface.print("{s}", .{t});
    try writer.flush();

    // Dir.writeFile(dir: Dir, io: Io, options: WriteFileOptions)

    // std.debug.print("{s}", .{t});

    // for (ms) |m| {
    //     m.print();
    // }
}
