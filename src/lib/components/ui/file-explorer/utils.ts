interface LocalFile {
	filename: string;
	size?: number;
	lastModified: string;
	type?: string;
	typeLong?: string;
}

import ImageIcon from './icons/img_icon.png';
import TextIcon from './icons/text_icon.png';
import FolderIcon from './icons/folder_icon.png';
import ZipIcon from './icons/zip_icon.png';
import DocIcon from './icons/word_icon.png';
import PptIcon from './icons/ppt_icon.png';
import XlsIcon from './icons/excel_icon.png';
import PdfIcon from './icons/pdf_icon.png';
import ExeIcon from './icons/exe_icon.png';
import BackIcon from './icons/back_icon.png';


export function typeToIcon(type: string) {
	switch (type) {
		case "image": return ImageIcon;
		case "text": return TextIcon;
		case "folder": return FolderIcon;
		case "zip": return ZipIcon;
		case "doc": return DocIcon;
		case "ppt": return PptIcon;
		case "xls": return XlsIcon;
		case "pdf": return PdfIcon;
		case "exe": return ExeIcon;
		case "back": return BackIcon;
	}
	return null;
}

export function sizeToString(size: number) {
	let tmpSize = size;
	if (tmpSize < 1000) {
		return `${tmpSize} B`;
	}
	tmpSize = Math.round(tmpSize / 100) / 10;
	if (tmpSize < 1000) {
		return `${tmpSize} kB`;
	}
	tmpSize = Math.round(tmpSize / 100) / 10;
	if (tmpSize < 1000) {
		return `${tmpSize} MB`;
	}
	tmpSize = Math.round(tmpSize / 1000) / 10;
	return `${tmpSize} GB`;
}

export type { LocalFile };