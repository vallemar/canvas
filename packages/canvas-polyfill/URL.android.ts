import { knownFolders, File as NSFile, isIOS, path, Folder, Application, Utils } from '@nativescript/core';
const BLOB_PATH = 'blob:nativescript/';
const BLOB_DIR = 'ns_blobs';
const BLOB_KEYS = 'org.nativescript.canvas.blob.keys';
declare const org;

const MIME_TO_EXT = {
	'image/jpeg': '.jpg',
	'image/png': '.png',
	'image/gif': '.gif',
};

function get_mime_ext(value): string {
	return '.' + MIME_TO_EXT[value] ?? '';
}

let sharedPreferences;
interface BlobItem {
	blob: Blob;
	path?: string;
	type?: string;
	ext?: string;
}
const BLOB_STORE = new Map<string, BlobItem>();

import './url-search';
/*
export class URL {
	_native: java.net.URI;
	_isBlobURL = false;
	constructor(url: string, base?: string | URL) {
		if (url?.startsWith?.('blob:')) {
			this._isBlobURL = true;
		}
		let baseUrl: java.net.URL;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			this._native = new java.net.URI(url);
		} else {
			if (base instanceof URL) {
				baseUrl = base._native.toURL();
			} else if (base) {
				try {
					baseUrl = new java.net.URL(base);
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					this._native = new java.net.URL(baseUrl, url).toURI();
				} else {
					this._native = new java.net.URL(url).toURI();
				}
			} catch (e) {
				throw new TypeError(`Failed to construct 'URL': Invalid URL`);
			}
		}
	}

	get native() {
		return this._native.toURL();
	}

	private _updateURL() {
		const currentURL = this.native;
		this._native = new java.net.URI(this._protocol || currentURL.getProtocol(), `${this._username}${this.password ? ':' : ''}${this._password}` || currentURL.getUserInfo(), this._hostname || currentURL.getHost(), this._port || currentURL.getPort(), this._pathname || currentURL.getPath(), this._search || currentURL.getQuery(), this._hash || currentURL.getRef());
	}

	get hash() {
		const hash = this._native.getFragment();
		return hash ? `#${hash}` : '';
	}

	_hash = '';
	set hash(value: string) {
		this._hash = value;
		this._updateURL();
	}

	get host() {
		const port = this._native.getPort();
		const scheme = this.protocol;
		let returnPort = true;
		if (scheme === 'https' && port === 443) {
			returnPort = false;
		}
		return `${this._native.getHost()}${returnPort && port != -1 ? ':' : ''}${returnPort && port != -1 ? port : ''}`;
	}

	set host(value: string) {
		const url = new java.net.URL(value);
		this._port = url.getPort();
		this._hostname = url.getHost();
		this._updateURL();
	}

	get hostname() {
		return this._native.getHost();
	}

	_hostname = '';
	set hostname(value: string) {
		this._hostname = value;
		this._updateURL();
	}

	get href() {
		return this._native.toString();
	}

	set href(value: string) {
		this._native = new java.net.URI(value);
	}

	get origin() {
		let url = this._native;
		if (this._isBlobURL) {
			url = new java.net.URI(this._native.toString().replace('blob:', ''));
		}

		return `${url.getScheme()}://${url.getHost()}`;
	}

	_password = '';
	get password() {
		const user = this._native.getUserInfo() || '';
		return user.split(':')[1] || '';
	}

	set password(value: string) {
		this._password = value;
		this._updateURL();
	}

	get pathname() {
		return this._native.getPath();
	}

	_pathname = '';
	set pathname(value: string) {
		this._pathname = value;
		this._updateURL();
	}

	get port() {
		const port = this._native.getPort();
		if (port === -1) {
			return '';
		}
		return `${port}`;
	}

	_port = -1;
	set port(value: string) {
		this._port = +value;
		this._updateURL();
	}

	get protocol() {
		return this._native.getScheme() + ':';
	}

	_protocol = '';
	set protocol(value: string) {
		this._protocol = value;
		this._updateURL();
	}

	get search() {
		const query = this._native.getQuery();
		return query ? `?${query}` : '';
	}

	_search = '';
	set search(value: string) {
		this._search = value;
		this._updateURL();
	}

	get searchParams() {
		return new URLSearchParams(this._native.toString());
	}

	get username() {
		const user = this._native.getUserInfo() || '';
		return user.split(':')[0] || '';
	}

	_username = '';
	set username(value: string) {
		this.username = value;
		this._updateURL();
	}

	toJSON() {
		return this._native.toString();
	}

	toString() {
		return this._native.toString();
	}

	public static createObjectURL(object: any, options = null): string {
		//const buf = (Blob as any).InternalAccessor.getBuffer(object);
		if (object instanceof Blob || object instanceof File) {
			const id = java.util.UUID.randomUUID().toString();
			const ret = `blob:nativescript/${id}`;
			BLOB_STORE.set(ret, {
				blob: object,
				type: object?.type,
				ext: options?.ext,
			});
			return ret;
			//	return org.nativescript.canvas.polyfill.Utils.createObjectURL(Utils.android.getApplicationContext(), buf, buf.byteOffset, object?.type ?? null, options?.ext ?? null);
		}
		return null;
	}

	public static revokeObjectURL(url: string) {
		const blob = BLOB_STORE.get(url);
		if (blob.path) {
			org.nativescript.canvas.polyfill.Utils.revokeObjectURL(Utils.android.getApplicationContext(), url ?? null);
		}
		BLOB_STORE.delete(url);
		//org.nativescript.canvas.polyfill.Utils.revokeObjectURL(Utils.android.getApplicationContext(), url ?? null);
	}

	public static InternalAccessor = class {
		public static getPath(url: string) {
			//	return org.nativescript.canvas.polyfill.Utils.getPath(Utils.android.getApplicationContext(), url ?? null);
			const blob = BLOB_STORE.get(url);
			if (!blob) {
				return '';
			}
			if (blob.path) {
				return blob.path;
			}
			const buf = (Blob as any).InternalAccessor.getBuffer(blob.blob);
			const path = org.nativescript.canvas.polyfill.Utils.getItemOrCreateAndReturn(Utils.android.getApplicationContext(), url, buf, buf.byteOffset, blob?.type ?? null, blob?.ext ?? null);
			blob.path = path;
			BLOB_STORE.set(url, blob);
			return path;
		}

		public static getData(url: string) {
			return BLOB_STORE.get(url);
		}
	};
}
*/

export class URL {
	_native;
	_isBlobURL = false;
	constructor(url: string, base?: string | URL) {
		if (url?.startsWith?.('blob:')) {
			this._isBlobURL = true;
		}
		let baseUrl: string;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			this._native = new global.CanvasModule.URL(url);
		} else {
			if (base instanceof URL) {
				baseUrl = base._native.toString();
			} else if (base) {
				try {
					baseUrl = base.toString();
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					this._native = new global.CanvasModule.URL(url, baseUrl);
				} else {
					this._native = new global.CanvasModule.URL(url);
				}
			} catch (e) {
				throw new TypeError(`Failed to construct 'URL': Invalid URL`);
			}
		}
	}

	get native() {
		return this._native;
	}

	get hash() {
		return this.native.hash;
	}

	set hash(value: string) {
		this.native.hash = value;
	}

	get host() {
		return this.native.host;
	}

	set host(value: string) {
		this.native.host = value;
	}

	get hostname() {
		return this.native.hostname;
	}

	set hostname(value: string) {
		this.native.hostname = value;
	}

	get href() {
		return this.native.href;
	}

	set href(value: string) {
		this.native.href = value;
	}

	get origin() {
		// let url = this._native;
		// if (this._isBlobURL) {
		// 	url = new java.net.URI(this._native.toString().replace('blob:', ''));
		// }

		// return `${url.getScheme()}://${url.getHost()}`;

		return this.native.origin;
	}

	get password() {
		return this.native.password;
	}

	set password(value: string) {
		this.native.password = value;
	}

	get pathname() {
		return this.native.pathname;
	}

	set pathname(value: string) {
		this.native.pathname = value;
	}

	get port() {
		return this.native.port;
	}

	set port(value: string) {
		this.native.port = value;
	}

	get protocol() {
		return this.native.protocol;
	}

	set protocol(value: string) {
		this.native.protocol = value;
	}

	get search() {
		return this.native.search;
	}

	set search(value: string) {
		this.native.search = value;
	}

	get searchParams() {
		return new URLSearchParams(this.native.toString());
	}

	get username() {
		return this.native.username;
	}

	set username(value: string) {
		this.native.username = value;
	}

	toJSON() {
		return this.native.toString();
	}

	toString() {
		return this.native.toString();
	}

	public static canParse(url, base) {
		let ret = false;
		if (url?.startsWith?.('blob:')) {
			ret = true;
		}
		let baseUrl: string;
		if (typeof url === 'string' && url.startsWith('blob:')) {
			ret = true;
		} else {
			if (base instanceof URL) {
				baseUrl = base._native.toString();
			} else if (base) {
				try {
					baseUrl = base.toString();
				} catch (e) {
					throw new TypeError(`Failed to construct 'URL': Invalid base URL`);
				}
			}
			try {
				if (baseUrl) {
					ret = global.CanvasModule.URL.canParse(url, baseUrl);
				} else {
					ret = global.CanvasModule.URL.canParse(url);
				}
			} catch (e) {}
		}

		return ret;
	}

	public static createObjectURL(object: any, options = null): string {
		//const buf = (Blob as any).InternalAccessor.getBuffer(object);
		if (object instanceof Blob || object instanceof File) {
			const id = java.util.UUID.randomUUID().toString();
			const ret = `blob:nativescript/${id}`;
			BLOB_STORE.set(ret, {
				blob: object,
				type: object?.type,
				ext: options?.ext,
			});
			return ret;
			//	return org.nativescript.canvas.polyfill.Utils.createObjectURL(Utils.android.getApplicationContext(), buf, buf.byteOffset, object?.type ?? null, options?.ext ?? null);
		}
		return null;
	}

	public static revokeObjectURL(url: string) {
		const blob = BLOB_STORE.get(url);
		if (blob.path) {
			org.nativescript.canvas.polyfill.Utils.revokeObjectURL(Utils.android.getApplicationContext(), url ?? null);
		}
		BLOB_STORE.delete(url);
		//org.nativescript.canvas.polyfill.Utils.revokeObjectURL(Utils.android.getApplicationContext(), url ?? null);
	}

	public static InternalAccessor = class {
		public static getPath(url: string) {
			//	return org.nativescript.canvas.polyfill.Utils.getPath(Utils.android.getApplicationContext(), url ?? null);
			const blob = BLOB_STORE.get(url);
			if (!blob) {
				return '';
			}
			if (blob.path) {
				return blob.path;
			}
			const buf = (Blob as any).InternalAccessor.getBuffer(blob.blob);
			const path = org.nativescript.canvas.polyfill.Utils.getItemOrCreateAndReturn(Utils.android.getApplicationContext(), url, buf, buf.byteOffset, blob?.type ?? null, blob?.ext ?? null);
			blob.path = path;
			BLOB_STORE.set(url, blob);
			return path;
		}

		public static getData(url: string) {
			return BLOB_STORE.get(url);
		}
	};
}
