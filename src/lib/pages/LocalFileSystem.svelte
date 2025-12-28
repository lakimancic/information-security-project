<script lang="ts">
	import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";

    const files : LocalFile[] = [
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'File Folder', type: 'folder'},
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'Text File', type: 'text'},
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, typeLong: 'Image File', type: 'image'},
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Presentation File', type: 'ppt'},
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, typeLong: 'Document File', type: 'doc'},
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Spreadsheet File', type: 'xls'},
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Compressed Folder', type: 'zip' },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'PDF Document', type: 'pdf'},
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Executable File', type: 'exe'},
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'File Folder', type: 'folder'},
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'Text File', type: 'text'},
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, typeLong: 'Image File', type: 'image'},
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Presentation File', type: 'ppt'},
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, typeLong: 'Document File', type: 'doc'},
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Spreadsheet File', type: 'xls'},
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Compressed Folder', type: 'zip' },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'PDF Document', type: 'pdf'},
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Executable File', type: 'exe'},
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'File Folder', type: 'folder'},
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'Text File', type: 'text'},
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, typeLong: 'Image File', type: 'image'},
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Presentation File', type: 'ppt'},
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, typeLong: 'Document File', type: 'doc'},
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Spreadsheet File', type: 'xls'},
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Compressed Folder', type: 'zip' },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'PDF Document', type: 'pdf'},
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Executable File', type: 'exe'},
    ];
</script>

<div class="flex flex-1 min-h-0">
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer class="" pwd="/home/lazarm" files={files} />
    </div>
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer class="" pwd="/home/lazarm" files={files} />
    </div>
</div>