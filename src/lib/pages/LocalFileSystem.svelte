<script lang="ts">
	import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";

    const files : LocalFile[] = [
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, type: 'File Folder', selected: false },
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, type: 'Text File', selected: false },
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, type: 'Image File', selected: false },
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, type: 'Presentation File', selected: false },
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, type: 'Document File', selected: false },
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, type: 'Spreadsheet File', selected: false },
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, type: 'Compressed Folder', selected: true },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, type: 'PDF Document', selected: false },
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, type: 'Executable File', selected: false },
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, type: 'File Folder', selected: false },
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, type: 'Text File', selected: false },
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, type: 'Image File', selected: false },
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, type: 'Presentation File', selected: false },
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, type: 'Document File', selected: false },
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, type: 'Spreadsheet File', selected: false },
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, type: 'Compressed Folder', selected: true },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, type: 'PDF Document', selected: false },
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, type: 'Executable File', selected: false },
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, type: 'File Folder', selected: false },
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, type: 'Text File', selected: false },
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, type: 'Image File', selected: false },
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, type: 'Presentation File', selected: false },
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, type: 'Document File', selected: false },
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, type: 'Spreadsheet File', selected: false },
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, type: 'Compressed Folder', selected: true },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, type: 'PDF Document', selected: false },
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, type: 'Executable File', selected: false },
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