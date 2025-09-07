
// type definition for a plugin environment
declare type Environment = {
	// the identifier used internally to manage a plugin
	id: string;
} & Record<string, any>;

/** Return value for a key lookup
 *
 */
type Value = object;

/** stores an object against the key in state. The state is persisted to Disk
 * and will be available after a restart of jilebi
 *
 * ```ts
 * setState(env, "knowledgeGraph", knowledgeGraph)
 * 
 * ```
 *
 * Returns false if the state cannot be set
 *
 * Requires no permissions.
 *
 * @tags state
 * @category State Management
 */
declare function setState(env: Environment, key: string, value: object): boolean;



/** retrieves the object stored against the key in state.
 *
 * ```ts
 * getState(env, "knowledgeGraph")
 * 
 * ```
 *
 * Returns null if the state cannot be retrieved
 *
 * Requires no permissions.
 *
 * @tags state
 * @category State Management
 */
declare function getState(env: Environment, key: string): Value | null;


/** deletes the object stored against a key in state
 *
 * ```ts
 * deleteState(env, "knowledgeGraph")
 * 
 * ```
 *
 * Returns false if the state cannot be deleted
 *
 * Requires no permissions.
 *
 * @tags state
 * @category State Management
 */
declare function deleteState(env: Environment, key: string): boolean;

// type definitions for deno extensions
declare namespace Deno {
	/**
	 * Options which can be set when doing {@linkcode Deno.open} and
	 * {@linkcode Deno.openSync}.
	 *
	 * @category File System */
	export interface OpenOptions {
		/** Sets the option for read access. This option, when `true`, means that
		 * the file should be read-able if opened.
		 *
		 * @default {true} */
		read?: boolean;
		/** Sets the option for write access. This option, when `true`, means that
		 * the file should be write-able if opened. If the file already exists,
		 * any write calls on it will overwrite its contents, by default without
		 * truncating it.
		 *
		 * @default {false} */
		write?: boolean;
		/** Sets the option for the append mode. This option, when `true`, means
		 * that writes will append to a file instead of overwriting previous
		 * contents.
		 *
		 * Note that setting `{ write: true, append: true }` has the same effect as
		 * setting only `{ append: true }`.
		 *
		 * @default {false} */
		append?: boolean;
		/** Sets the option for truncating a previous file. If a file is
		 * successfully opened with this option set it will truncate the file to `0`
		 * size if it already exists. The file must be opened with write access
		 * for truncate to work.
		 *
		 * @default {false} */
		truncate?: boolean;
		/** Sets the option to allow creating a new file, if one doesn't already
		 * exist at the specified path. Requires write or append access to be
		 * used.
		 *
		 * @default {false} */
		create?: boolean;
		/** If set to `true`, no file, directory, or symlink is allowed to exist at
		 * the target location. Requires write or append access to be used. When
		 * createNew is set to `true`, create and truncate are ignored.
		 *
		 * @default {false} */
		createNew?: boolean;
		/** Permissions to use if creating the file (defaults to `0o666`, before
		 * the process's umask).
		 *
		 * Ignored on Windows. */
		mode?: number;
	}

	/**
	 * Options which can be set when using {@linkcode Deno.readFile} or
	 * {@linkcode Deno.readFileSync}.
	 *
	 * @category File System */
	export interface ReadFileOptions {
		/**
		 * An abort signal to allow cancellation of the file read operation.
		 * If the signal becomes aborted the readFile operation will be stopped
		 * and the promise returned will be rejected with an AbortError.
		 */
		signal?: AbortSignal;
	}

	/**
	 * Options which can be set when using {@linkcode Deno.mkdir} and
	 * {@linkcode Deno.mkdirSync}.
	 *
	 * @category File System */
	export interface MkdirOptions {
		/** If set to `true`, means that any intermediate directories will also be
		 * created (as with the shell command `mkdir -p`).
		 *
		 * Intermediate directories are created with the same permissions.
		 *
		 * When recursive is set to `true`, succeeds silently (without changing any
		 * permissions) if a directory already exists at the path, or if the path
		 * is a symlink to an existing directory.
		 *
		 * @default {false} */
		recursive?: boolean;
		/** Permissions to use when creating the directory (defaults to `0o777`,
		 * before the process's umask).
		 *
		 * Ignored on Windows. */
		mode?: number;
	}

	/** Creates a new directory with the specified path.
	 *
	 * ```ts
	 * await Deno.mkdir("new_dir");
	 * await Deno.mkdir("nested/directories", { recursive: true });
	 * await Deno.mkdir("restricted_access_dir", { mode: 0o700 });
	 * ```
	 *
	 * Defaults to throwing error if the directory already exists.
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function mkdir(
		path: string | URL,
		options?: MkdirOptions,
	): Promise<void>;

	/** Synchronously creates a new directory with the specified path.
	 *
	 * ```ts
	 * Deno.mkdirSync("new_dir");
	 * Deno.mkdirSync("nested/directories", { recursive: true });
	 * Deno.mkdirSync("restricted_access_dir", { mode: 0o700 });
	 * ```
	 *
	 * Defaults to throwing error if the directory already exists.
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function mkdirSync(path: string | URL, options?: MkdirOptions): void;

	/**
	 * Options which can be set when using {@linkcode Deno.makeTempDir},
	 * {@linkcode Deno.makeTempDirSync}, {@linkcode Deno.makeTempFile}, and
	 * {@linkcode Deno.makeTempFileSync}.
	 *
	 * @category File System */
	export interface MakeTempOptions {
		/** Directory where the temporary directory should be created (defaults to
		 * the env variable `TMPDIR`, or the system's default, usually `/tmp`).
		 *
		 * Note that if the passed `dir` is relative, the path returned by
		 * `makeTempFile()` and `makeTempDir()` will also be relative. Be mindful of
		 * this when changing working directory. */
		dir?: string;
		/** String that should precede the random portion of the temporary
		 * directory's name. */
		prefix?: string;
		/** String that should follow the random portion of the temporary
		 * directory's name. */
		suffix?: string;
	}

	/** Creates a new temporary directory in the default directory for temporary
	 * files, unless `dir` is specified. Other optional options include
	 * prefixing and suffixing the directory name with `prefix` and `suffix`
	 * respectively.
	 *
	 * This call resolves to the full path to the newly created directory.
	 *
	 * Multiple programs calling this function simultaneously will create different
	 * directories. It is the caller's responsibility to remove the directory when
	 * no longer needed.
	 *
	 * ```ts
	 * const tempDirName0 = await Deno.makeTempDir();  // e.g. /tmp/2894ea76
	 * const tempDirName1 = await Deno.makeTempDir({ prefix: 'my_temp' }); // e.g. /tmp/my_temp339c944d
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	// TODO(ry) Doesn't check permissions.
	export function makeTempDir(options?: MakeTempOptions): Promise<string>;

	/** Synchronously creates a new temporary directory in the default directory
	 * for temporary files, unless `dir` is specified. Other optional options
	 * include prefixing and suffixing the directory name with `prefix` and
	 * `suffix` respectively.
	 *
	 * The full path to the newly created directory is returned.
	 *
	 * Multiple programs calling this function simultaneously will create different
	 * directories. It is the caller's responsibility to remove the directory when
	 * no longer needed.
	 *
	 * ```ts
	 * const tempDirName0 = Deno.makeTempDirSync();  // e.g. /tmp/2894ea76
	 * const tempDirName1 = Deno.makeTempDirSync({ prefix: 'my_temp' });  // e.g. /tmp/my_temp339c944d
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	// TODO(ry) Doesn't check permissions.
	export function makeTempDirSync(options?: MakeTempOptions): string;

	/** Creates a new temporary file in the default directory for temporary
	 * files, unless `dir` is specified.
	 *
	 * Other options include prefixing and suffixing the directory name with
	 * `prefix` and `suffix` respectively.
	 *
	 * This call resolves to the full path to the newly created file.
	 *
	 * Multiple programs calling this function simultaneously will create
	 * different files. It is the caller's responsibility to remove the file when
	 * no longer needed.
	 *
	 * ```ts
	 * const tmpFileName0 = await Deno.makeTempFile();  // e.g. /tmp/419e0bf2
	 * const tmpFileName1 = await Deno.makeTempFile({ prefix: 'my_temp' });  // e.g. /tmp/my_temp754d3098
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function makeTempFile(options?: MakeTempOptions): Promise<string>;

	/** Synchronously creates a new temporary file in the default directory for
	 * temporary files, unless `dir` is specified.
	 *
	 * Other options include prefixing and suffixing the directory name with
	 * `prefix` and `suffix` respectively.
	 *
	 * The full path to the newly created file is returned.
	 *
	 * Multiple programs calling this function simultaneously will create
	 * different files. It is the caller's responsibility to remove the file when
	 * no longer needed.
	 *
	 * ```ts
	 * const tempFileName0 = Deno.makeTempFileSync(); // e.g. /tmp/419e0bf2
	 * const tempFileName1 = Deno.makeTempFileSync({ prefix: 'my_temp' });  // e.g. /tmp/my_temp754d3098
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function makeTempFileSync(options?: MakeTempOptions): string;

	/** Changes the permission of a specific file/directory of specified path.
	 * Ignores the process's umask.
	 *
	 * ```ts
	 * await Deno.chmod("/path/to/file", 0o666);
	 * ```
	 *
	 * The mode is a sequence of 3 octal numbers. The first/left-most number
	 * specifies the permissions for the owner. The second number specifies the
	 * permissions for the group. The last/right-most number specifies the
	 * permissions for others. For example, with a mode of 0o764, the owner (7)
	 * can read/write/execute, the group (6) can read/write and everyone else (4)
	 * can read only.
	 *
	 * | Number | Description |
	 * | ------ | ----------- |
	 * | 7      | read, write, and execute |
	 * | 6      | read and write |
	 * | 5      | read and execute |
	 * | 4      | read only |
	 * | 3      | write and execute |
	 * | 2      | write only |
	 * | 1      | execute only |
	 * | 0      | no permission |
	 *
	 * NOTE: This API currently throws on Windows
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function chmod(path: string | URL, mode: number): Promise<void>;

	/** Synchronously changes the permission of a specific file/directory of
	 * specified path. Ignores the process's umask.
	 *
	 * ```ts
	 * Deno.chmodSync("/path/to/file", 0o666);
	 * ```
	 *
	 * For a full description, see {@linkcode Deno.chmod}.
	 *
	 * NOTE: This API currently throws on Windows
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function chmodSync(path: string | URL, mode: number): void;

	/** Change owner of a regular file or directory.
	 *
	 * This functionality is not available on Windows.
	 *
	 * ```ts
	 * await Deno.chown("myFile.txt", 1000, 1002);
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * Throws Error (not implemented) if executed on Windows.
	 *
	 * @tags allow-write
	 * @category File System
	 *
	 * @param path path to the file
	 * @param uid user id (UID) of the new owner, or `null` for no change
	 * @param gid group id (GID) of the new owner, or `null` for no change
	 */
	export function chown(
		path: string | URL,
		uid: number | null,
		gid: number | null,
	): Promise<void>;

	/** Synchronously change owner of a regular file or directory.
	 *
	 * This functionality is not available on Windows.
	 *
	 * ```ts
	 * Deno.chownSync("myFile.txt", 1000, 1002);
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * Throws Error (not implemented) if executed on Windows.
	 *
	 * @tags allow-write
	 * @category File System
	 *
	 * @param path path to the file
	 * @param uid user id (UID) of the new owner, or `null` for no change
	 * @param gid group id (GID) of the new owner, or `null` for no change
	 */
	export function chownSync(
		path: string | URL,
		uid: number | null,
		gid: number | null,
	): void;

	/**
	 * Options which can be set when using {@linkcode Deno.remove} and
	 * {@linkcode Deno.removeSync}.
	 *
	 * @category File System */
	export interface RemoveOptions {
		/** If set to `true`, path will be removed even if it's a non-empty directory.
		 *
		 * @default {false} */
		recursive?: boolean;
	}

	/** Removes the named file or directory.
	 *
	 * ```ts
	 * await Deno.remove("/path/to/empty_dir/or/file");
	 * await Deno.remove("/path/to/populated_dir/or/file", { recursive: true });
	 * ```
	 *
	 * Throws error if permission denied, path not found, or path is a non-empty
	 * directory and the `recursive` option isn't set to `true`.
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function remove(
		path: string | URL,
		options?: RemoveOptions,
	): Promise<void>;

	/** Synchronously removes the named file or directory.
	 *
	 * ```ts
	 * Deno.removeSync("/path/to/empty_dir/or/file");
	 * Deno.removeSync("/path/to/populated_dir/or/file", { recursive: true });
	 * ```
	 *
	 * Throws error if permission denied, path not found, or path is a non-empty
	 * directory and the `recursive` option isn't set to `true`.
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function removeSync(path: string | URL, options?: RemoveOptions): void;

	/** Synchronously renames (moves) `oldpath` to `newpath`. Paths may be files or
	 * directories. If `newpath` already exists and is not a directory,
	 * `renameSync()` replaces it. OS-specific restrictions may apply when
	 * `oldpath` and `newpath` are in different directories.
	 *
	 * ```ts
	 * Deno.renameSync("old/path", "new/path");
	 * ```
	 *
	 * On Unix-like OSes, this operation does not follow symlinks at either path.
	 *
	 * It varies between platforms when the operation throws errors, and if so what
	 * they are. It's always an error to rename anything to a non-empty directory.
	 *
	 * Requires `allow-read` and `allow-write` permissions.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function renameSync(
		oldpath: string | URL,
		newpath: string | URL,
	): void;

	/** Renames (moves) `oldpath` to `newpath`. Paths may be files or directories.
	 * If `newpath` already exists and is not a directory, `rename()` replaces it.
	 * OS-specific restrictions may apply when `oldpath` and `newpath` are in
	 * different directories.
	 *
	 * ```ts
	 * await Deno.rename("old/path", "new/path");
	 * ```
	 *
	 * On Unix-like OSes, this operation does not follow symlinks at either path.
	 *
	 * It varies between platforms when the operation throws errors, and if so
	 * what they are. It's always an error to rename anything to a non-empty
	 * directory.
	 *
	 * Requires `allow-read` and `allow-write` permissions.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function rename(
		oldpath: string | URL,
		newpath: string | URL,
	): Promise<void>;

	/** Asynchronously reads and returns the entire contents of a file as an UTF-8
	 *  decoded string. Reading a directory throws an error.
	 *
	 * ```ts
	 * const data = await Deno.readTextFile("hello.txt");
	 * console.log(data);
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readTextFile(
		path: string | URL,
		options?: ReadFileOptions,
	): Promise<string>;

	/** Synchronously reads and returns the entire contents of a file as an UTF-8
	 *  decoded string. Reading a directory throws an error.
	 *
	 * ```ts
	 * const data = Deno.readTextFileSync("hello.txt");
	 * console.log(data);
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readTextFileSync(path: string | URL): string;

	/** Reads and resolves to the entire contents of a file as an array of bytes.
	 * `TextDecoder` can be used to transform the bytes to string if required.
	 * Rejects with an error when reading a directory.
	 *
	 * ```ts
	 * const decoder = new TextDecoder("utf-8");
	 * const data = await Deno.readFile("hello.txt");
	 * console.log(decoder.decode(data));
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readFile(
		path: string | URL,
		options?: ReadFileOptions,
	): Promise<Uint8Array<ArrayBuffer>>;

	/** Synchronously reads and returns the entire contents of a file as an array
	 * of bytes. `TextDecoder` can be used to transform the bytes to string if
	 * required. Throws an error when reading a directory.
	 *
	 * ```ts
	 * const decoder = new TextDecoder("utf-8");
	 * const data = Deno.readFileSync("hello.txt");
	 * console.log(decoder.decode(data));
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readFileSync(path: string | URL): Uint8Array<ArrayBuffer>;

	/** Provides information about a file and is returned by
	 * {@linkcode Deno.stat}, {@linkcode Deno.lstat}, {@linkcode Deno.statSync},
	 * and {@linkcode Deno.lstatSync} or from calling `stat()` and `statSync()`
	 * on an {@linkcode Deno.FsFile} instance.
	 *
	 * @category File System
	 */
	export interface FileInfo {
		/** True if this is info for a regular file. Mutually exclusive to
		 * `FileInfo.isDirectory` and `FileInfo.isSymlink`. */
		isFile: boolean;
		/** True if this is info for a regular directory. Mutually exclusive to
		 * `FileInfo.isFile` and `FileInfo.isSymlink`. */
		isDirectory: boolean;
		/** True if this is info for a symlink. Mutually exclusive to
		 * `FileInfo.isFile` and `FileInfo.isDirectory`. */
		isSymlink: boolean;
		/** The size of the file, in bytes. */
		size: number;
		/** The last modification time of the file. This corresponds to the `mtime`
		 * field from `stat` on Linux/Mac OS and `ftLastWriteTime` on Windows. This
		 * may not be available on all platforms. */
		mtime: Date | null;
		/** The last access time of the file. This corresponds to the `atime`
		 * field from `stat` on Unix and `ftLastAccessTime` on Windows. This may not
		 * be available on all platforms. */
		atime: Date | null;
		/** The creation time of the file. This corresponds to the `birthtime`
		 * field from `stat` on Mac/BSD and `ftCreationTime` on Windows. This may
		 * not be available on all platforms. */
		birthtime: Date | null;
		/** The last change time of the file. This corresponds to the `ctime`
		 * field from `stat` on Mac/BSD and `ChangeTime` on Windows. This may
		 * not be available on all platforms. */
		ctime: Date | null;
		/** ID of the device containing the file. */
		dev: number;
		/** Inode number.
		 *
		 * _Linux/Mac OS only._ */
		ino: number | null;
		/** The underlying raw `st_mode` bits that contain the standard Unix
		 * permissions for this file/directory.
		 */
		mode: number | null;
		/** Number of hard links pointing to this file.
		 *
		 * _Linux/Mac OS only._ */
		nlink: number | null;
		/** User ID of the owner of this file.
		 *
		 * _Linux/Mac OS only._ */
		uid: number | null;
		/** Group ID of the owner of this file.
		 *
		 * _Linux/Mac OS only._ */
		gid: number | null;
		/** Device ID of this file.
		 *
		 * _Linux/Mac OS only._ */
		rdev: number | null;
		/** Blocksize for filesystem I/O.
		 *
		 * _Linux/Mac OS only._ */
		blksize: number | null;
		/** Number of blocks allocated to the file, in 512-byte units.
		 *
		 * _Linux/Mac OS only._ */
		blocks: number | null;
		/**  True if this is info for a block device.
		 *
		 * _Linux/Mac OS only._ */
		isBlockDevice: boolean | null;
		/**  True if this is info for a char device.
		 *
		 * _Linux/Mac OS only._ */
		isCharDevice: boolean | null;
		/**  True if this is info for a fifo.
		 *
		 * _Linux/Mac OS only._ */
		isFifo: boolean | null;
		/**  True if this is info for a socket.
		 *
		 * _Linux/Mac OS only._ */
		isSocket: boolean | null;
	}

	/** Resolves to the absolute normalized path, with symbolic links resolved.
	 *
	 * ```ts
	 * // e.g. given /home/alice/file.txt and current directory /home/alice
	 * await Deno.symlink("file.txt", "symlink_file.txt");
	 * const realPath = await Deno.realPath("./file.txt");
	 * const realSymLinkPath = await Deno.realPath("./symlink_file.txt");
	 * console.log(realPath);  // outputs "/home/alice/file.txt"
	 * console.log(realSymLinkPath);  // outputs "/home/alice/file.txt"
	 * ```
	 *
	 * Requires `allow-read` permission for the target path.
	 *
	 * Also requires `allow-read` permission for the `CWD` if the target path is
	 * relative.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function realPath(path: string | URL): Promise<string>;

	/** Synchronously returns absolute normalized path, with symbolic links
	 * resolved.
	 *
	 * ```ts
	 * // e.g. given /home/alice/file.txt and current directory /home/alice
	 * Deno.symlinkSync("file.txt", "symlink_file.txt");
	 * const realPath = Deno.realPathSync("./file.txt");
	 * const realSymLinkPath = Deno.realPathSync("./symlink_file.txt");
	 * console.log(realPath);  // outputs "/home/alice/file.txt"
	 * console.log(realSymLinkPath);  // outputs "/home/alice/file.txt"
	 * ```
	 *
	 * Requires `allow-read` permission for the target path.
	 *
	 * Also requires `allow-read` permission for the `CWD` if the target path is
	 * relative.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function realPathSync(path: string | URL): string;

	/**
	 * Information about a directory entry returned from {@linkcode Deno.readDir}
	 * and {@linkcode Deno.readDirSync}.
	 *
	 * @category File System */
	export interface DirEntry {
		/** The file name of the entry. It is just the entity name and does not
		 * include the full path. */
		name: string;
		/** True if this is info for a regular file. Mutually exclusive to
		 * `DirEntry.isDirectory` and `DirEntry.isSymlink`. */
		isFile: boolean;
		/** True if this is info for a regular directory. Mutually exclusive to
		 * `DirEntry.isFile` and `DirEntry.isSymlink`. */
		isDirectory: boolean;
		/** True if this is info for a symlink. Mutually exclusive to
		 * `DirEntry.isFile` and `DirEntry.isDirectory`. */
		isSymlink: boolean;
	}

	/** Reads the directory given by `path` and returns an async iterable of
	 * {@linkcode Deno.DirEntry}. The order of entries is not guaranteed.
	 *
	 * ```ts
	 * for await (const dirEntry of Deno.readDir("/")) {
	 *   console.log(dirEntry.name);
	 * }
	 * ```
	 *
	 * Throws error if `path` is not a directory.
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readDir(path: string | URL): AsyncIterable<DirEntry>;

	/** Synchronously reads the directory given by `path` and returns an iterable
	 * of {@linkcode Deno.DirEntry}. The order of entries is not guaranteed.
	 *
	 * ```ts
	 * for (const dirEntry of Deno.readDirSync("/")) {
	 *   console.log(dirEntry.name);
	 * }
	 * ```
	 *
	 * Throws error if `path` is not a directory.
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readDirSync(path: string | URL): IteratorObject<DirEntry>;

	/** Copies the contents and permissions of one file to another specified path,
	 * by default creating a new file if needed, else overwriting. Fails if target
	 * path is a directory or is unwritable.
	 *
	 * ```ts
	 * await Deno.copyFile("from.txt", "to.txt");
	 * ```
	 *
	 * Requires `allow-read` permission on `fromPath`.
	 *
	 * Requires `allow-write` permission on `toPath`.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function copyFile(
		fromPath: string | URL,
		toPath: string | URL,
	): Promise<void>;

	/** Synchronously copies the contents and permissions of one file to another
	 * specified path, by default creating a new file if needed, else overwriting.
	 * Fails if target path is a directory or is unwritable.
	 *
	 * ```ts
	 * Deno.copyFileSync("from.txt", "to.txt");
	 * ```
	 *
	 * Requires `allow-read` permission on `fromPath`.
	 *
	 * Requires `allow-write` permission on `toPath`.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function copyFileSync(
		fromPath: string | URL,
		toPath: string | URL,
	): void;

	/** Resolves to the full path destination of the named symbolic link.
	 *
	 * ```ts
	 * await Deno.symlink("./test.txt", "./test_link.txt");
	 * const target = await Deno.readLink("./test_link.txt"); // full path of ./test.txt
	 * ```
	 *
	 * Throws TypeError if called with a hard link.
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readLink(path: string | URL): Promise<string>;

	/** Synchronously returns the full path destination of the named symbolic
	 * link.
	 *
	 * ```ts
	 * Deno.symlinkSync("./test.txt", "./test_link.txt");
	 * const target = Deno.readLinkSync("./test_link.txt"); // full path of ./test.txt
	 * ```
	 *
	 * Throws TypeError if called with a hard link.
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function readLinkSync(path: string | URL): string;

	/** Resolves to a {@linkcode Deno.FileInfo} for the specified `path`. If
	 * `path` is a symlink, information for the symlink will be returned instead
	 * of what it points to.
	 *
	 * ```ts
	 * import { assert } from "jsr:@std/assert";
	 * const fileInfo = await Deno.lstat("hello.txt");
	 * assert(fileInfo.isFile);
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function lstat(path: string | URL): Promise<FileInfo>;

	/** Synchronously returns a {@linkcode Deno.FileInfo} for the specified
	 * `path`. If `path` is a symlink, information for the symlink will be
	 * returned instead of what it points to.
	 *
	 * ```ts
	 * import { assert } from "jsr:@std/assert";
	 * const fileInfo = Deno.lstatSync("hello.txt");
	 * assert(fileInfo.isFile);
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function lstatSync(path: string | URL): FileInfo;

	/** Resolves to a {@linkcode Deno.FileInfo} for the specified `path`. Will
	 * always follow symlinks.
	 *
	 * ```ts
	 * import { assert } from "jsr:@std/assert";
	 * const fileInfo = await Deno.stat("hello.txt");
	 * assert(fileInfo.isFile);
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function stat(path: string | URL): Promise<FileInfo>;

	/** Synchronously returns a {@linkcode Deno.FileInfo} for the specified
	 * `path`. Will always follow symlinks.
	 *
	 * ```ts
	 * import { assert } from "jsr:@std/assert";
	 * const fileInfo = Deno.statSync("hello.txt");
	 * assert(fileInfo.isFile);
	 * ```
	 *
	 * Requires `allow-read` permission.
	 *
	 * @tags allow-read
	 * @category File System
	 */
	export function statSync(path: string | URL): FileInfo;

	/** Options for writing to a file.
	 *
	 * @category File System
	 */
	export interface WriteFileOptions {
		/** If set to `true`, will append to a file instead of overwriting previous
		 * contents.
		 *
		 * @default {false} */
		append?: boolean;
		/** Sets the option to allow creating a new file, if one doesn't already
		 * exist at the specified path.
		 *
		 * @default {true} */
		create?: boolean;
		/** If set to `true`, no file, directory, or symlink is allowed to exist at
		 * the target location. When createNew is set to `true`, `create` is ignored.
		 *
		 * @default {false} */
		createNew?: boolean;
		/** Permissions always applied to file. */
		mode?: number;
		/** An abort signal to allow cancellation of the file write operation.
		 *
		 * If the signal becomes aborted the write file operation will be stopped
		 * and the promise returned will be rejected with an {@linkcode AbortError}.
		 */
		signal?: AbortSignal;
	}

	/** Write `data` to the given `path`, by default creating a new file if
	 * needed, else overwriting.
	 *
	 * ```ts
	 * const encoder = new TextEncoder();
	 * const data = encoder.encode("Hello world\n");
	 * await Deno.writeFile("hello1.txt", data);  // overwrite "hello1.txt" or create it
	 * await Deno.writeFile("hello2.txt", data, { create: false });  // only works if "hello2.txt" exists
	 * await Deno.writeFile("hello3.txt", data, { mode: 0o777 });  // set permissions on new file
	 * await Deno.writeFile("hello4.txt", data, { append: true });  // add data to the end of the file
	 * ```
	 *
	 * Requires `allow-write` permission, and `allow-read` if `options.create` is
	 * `false`.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function writeFile(
		path: string | URL,
		data: Uint8Array | ReadableStream<Uint8Array>,
		options?: WriteFileOptions,
	): Promise<void>;

	/** Synchronously write `data` to the given `path`, by default creating a new
	 * file if needed, else overwriting.
	 *
	 * ```ts
	 * const encoder = new TextEncoder();
	 * const data = encoder.encode("Hello world\n");
	 * Deno.writeFileSync("hello1.txt", data);  // overwrite "hello1.txt" or create it
	 * Deno.writeFileSync("hello2.txt", data, { create: false });  // only works if "hello2.txt" exists
	 * Deno.writeFileSync("hello3.txt", data, { mode: 0o777 });  // set permissions on new file
	 * Deno.writeFileSync("hello4.txt", data, { append: true });  // add data to the end of the file
	 * ```
	 *
	 * Requires `allow-write` permission, and `allow-read` if `options.create` is
	 * `false`.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function writeFileSync(
		path: string | URL,
		data: Uint8Array,
		options?: WriteFileOptions,
	): void;

	/** Write string `data` to the given `path`, by default creating a new file if
	 * needed, else overwriting.
	 *
	 * ```ts
	 * await Deno.writeTextFile("hello1.txt", "Hello world\n");  // overwrite "hello1.txt" or create it
	 * ```
	 *
	 * Requires `allow-write` permission, and `allow-read` if `options.create` is
	 * `false`.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function writeTextFile(
		path: string | URL,
		data: string | ReadableStream<string>,
		options?: WriteFileOptions,
	): Promise<void>;

	/** Synchronously write string `data` to the given `path`, by default creating
	 * a new file if needed, else overwriting.
	 *
	 * ```ts
	 * Deno.writeTextFileSync("hello1.txt", "Hello world\n");  // overwrite "hello1.txt" or create it
	 * ```
	 *
	 * Requires `allow-write` permission, and `allow-read` if `options.create` is
	 * `false`.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function writeTextFileSync(
		path: string | URL,
		data: string,
		options?: WriteFileOptions,
	): void;

	/** Truncates (or extends) the specified file, to reach the specified `len`.
	 * If `len` is not specified then the entire file contents are truncated.
	 *
	 * ### Truncate the entire file
	 * ```ts
	 * await Deno.truncate("my_file.txt");
	 * ```
	 *
	 * ### Truncate part of the file
	 *
	 * ```ts
	 * const file = await Deno.makeTempFile();
	 * await Deno.writeTextFile(file, "Hello World");
	 * await Deno.truncate(file, 7);
	 * const data = await Deno.readFile(file);
	 * console.log(new TextDecoder().decode(data));  // "Hello W"
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function truncate(name: string, len?: number): Promise<void>;

	/** Synchronously truncates (or extends) the specified file, to reach the
	 * specified `len`. If `len` is not specified then the entire file contents
	 * are truncated.
	 *
	 * ### Truncate the entire file
	 *
	 * ```ts
	 * Deno.truncateSync("my_file.txt");
	 * ```
	 *
	 * ### Truncate part of the file
	 *
	 * ```ts
	 * const file = Deno.makeTempFileSync();
	 * Deno.writeFileSync(file, new TextEncoder().encode("Hello World"));
	 * Deno.truncateSync(file, 7);
	 * const data = Deno.readFileSync(file);
	 * console.log(new TextDecoder().decode(data));
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function truncateSync(name: string, len?: number): void;

	/** Options that can be used with {@linkcode symlink} and
	 * {@linkcode symlinkSync}.
	 *
	 * @category File System */
	export interface SymlinkOptions {
		/** Specify the symbolic link type as file, directory or NTFS junction. This
		 * option only applies to Windows and is ignored on other operating systems. */
		type: "file" | "dir" | "junction";
	}

	/**
	 * Creates `newpath` as a symbolic link to `oldpath`.
	 *
	 * The `options.type` parameter can be set to `"file"`, `"dir"` or `"junction"`.
	 * This argument is only available on Windows and ignored on other platforms.
	 *
	 * ```ts
	 * await Deno.symlink("old/name", "new/name");
	 * ```
	 *
	 * Requires full `allow-read` and `allow-write` permissions.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function symlink(
		oldpath: string | URL,
		newpath: string | URL,
		options?: SymlinkOptions,
	): Promise<void>;

	/**
	 * Creates `newpath` as a symbolic link to `oldpath`.
	 *
	 * The `options.type` parameter can be set to `"file"`, `"dir"` or `"junction"`.
	 * This argument is only available on Windows and ignored on other platforms.
	 *
	 * ```ts
	 * Deno.symlinkSync("old/name", "new/name");
	 * ```
	 *
	 * Requires full `allow-read` and `allow-write` permissions.
	 *
	 * @tags allow-read, allow-write
	 * @category File System
	 */
	export function symlinkSync(
		oldpath: string | URL,
		newpath: string | URL,
		options?: SymlinkOptions,
	): void;

	/**
	 * Synchronously changes the access (`atime`) and modification (`mtime`) times
	 * of a file system object referenced by `path`. Given times are either in
	 * seconds (UNIX epoch time) or as `Date` objects.
	 *
	 * ```ts
	 * Deno.utimeSync("myfile.txt", 1556495550, new Date());
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function utimeSync(
		path: string | URL,
		atime: number | Date,
		mtime: number | Date,
	): void;

	/**
	 * Changes the access (`atime`) and modification (`mtime`) times of a file
	 * system object referenced by `path`. Given times are either in seconds
	 * (UNIX epoch time) or as `Date` objects.
	 *
	 * ```ts
	 * await Deno.utime("myfile.txt", 1556495550, new Date());
	 * ```
	 *
	 * Requires `allow-write` permission.
	 *
	 * @tags allow-write
	 * @category File System
	 */
	export function utime(
		path: string | URL,
		atime: number | Date,
		mtime: number | Date,
	): Promise<void>;

	/** Retrieve the process umask.  If `mask` is provided, sets the process umask.
	 * This call always returns what the umask was before the call.
	 *
	 * ```ts
	 * console.log(Deno.umask());  // e.g. 18 (0o022)
	 * const prevUmaskValue = Deno.umask(0o077);  // e.g. 18 (0o022)
	 * console.log(Deno.umask());  // e.g. 63 (0o077)
	 * ```
	 *
	 * This API is under consideration to determine if permissions are required to
	 * call it.
	 *
	 * *Note*: This API is not implemented on Windows
	 *
	 * @category File System
	 */
	export function umask(mask?: number): number;
}
