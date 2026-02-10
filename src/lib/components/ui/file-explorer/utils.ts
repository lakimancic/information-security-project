interface LocalFile {
	filename: string;
	size?: number;
	lastModified: string;
	fileType?: string;
	typeLong?: string;
}

interface ProgressFile {
	filename: string;
	processed: number;
	total: number;
	size?: number;
}

interface PendingFile {
	filename: string;
	sockAddr: string;
	size: number;
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
import UnkIcon from './icons/unk_icon.png';

export function typeToIcon(type: string) {
	switch (type) {
		case 'image':
			return ImageIcon;
		case 'text':
			return TextIcon;
		case 'folder':
			return FolderIcon;
		case 'zip':
			return ZipIcon;
		case 'doc':
			return DocIcon;
		case 'ppt':
			return PptIcon;
		case 'xls':
			return XlsIcon;
		case 'pdf':
			return PdfIcon;
		case 'exe':
			return ExeIcon;
		case 'back':
			return BackIcon;
		default:
			return UnkIcon;
	}
	return null;
}

export function sizeToString(size: number) {
    if (size === 0) return "0 B";
    
    const units = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
    
    if (size < 1024) {
        return `${size} B`;
    }
    
    const i = Math.floor(Math.log2(size) / 10);
    const unit = units[Math.min(i, units.length - 1)];
    const value = size / Math.pow(1024, Math.min(i, units.length - 1));
    const rounded = Math.round(value * 10) / 10;
    const formatted = rounded % 1 === 0 ? rounded.toFixed(0) : rounded.toFixed(1);
    
    return `${formatted} ${unit}`;
}

export type { LocalFile, ProgressFile, PendingFile };
