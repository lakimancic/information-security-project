interface LocalFile {
	filename: string;
	size?: number;
	lastModified: string;
	type?: string;
	typeLong?: string;
	selected: boolean;
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


export function typeToIcon(type: string) {
	switch (type) {
		case "Image File": return ImageIcon;
		case "Text File": return TextIcon;
		case "File Folder": return FolderIcon;
		case "Compressed Folder": return ZipIcon;
		case "Document File": return DocIcon;
		case "Presentation File": return PptIcon;
		case "Spreadsheet File": return XlsIcon;
		case "PDF Document": return PdfIcon;
		case "Executable File": return ExeIcon;
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