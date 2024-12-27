<template>
  <ckeditor
    :editor="ClassicEditor"
    :config="config"
   />
</template>

<script lang="ts" setup>
// https://ckeditor.com/docs/ckeditor5/latest/examples/builds-custom/full-featured-editor.html
import { ref, computed } from 'vue';
import { ClassicEditor, Essentials, Title,
    FontSize, FontFamily, FontColor, FontBackgroundColor,
    Bold, Italic, Underline, Strikethrough, HorizontalLine, Code,
    RemoveFormat,
    Alignment,
    List, ListProperties,
    Indent, IndentBlock,
    Heading, Link, Table, TableToolbar, Highlight,
    BlockQuote, CodeBlock,
    SourceEditing,
} from 'ckeditor5';
import { Ckeditor } from '@ckeditor/ckeditor5-vue';

import 'ckeditor5/ckeditor5.css';

const config = computed( () => {
    return {
        licenseKey: 'GPL', // Or 'GPL'.
        plugins: [Essentials, Title,
            Heading,
            FontSize, FontFamily, FontColor, FontBackgroundColor,
            Bold, Italic, Underline, Strikethrough, HorizontalLine, Code,
            RemoveFormat,
            Alignment,
            List, ListProperties,
            Indent, IndentBlock,
            Link, Table, TableToolbar, Highlight,
            BlockQuote, CodeBlock,
            SourceEditing,
        ],
        toolbar: {
            items:[
                'undo', 'redo', '|',
                'heading', '|',
                'fontSize', 'fontFamily', 'fontColor', 'fontBackgroundColor', '|',
                'bold', 'italic', 'underline', 'strikethrough', 'code', 'horizontalLine', '|',
                'removeFormat', '|',
                'alignment', '|',
                'bulletedList', 'numberedList', '|',
                'indent', 'outdent', '|',
                'link', 'insertTable', 'highlight', '|',
                'blockQuote', 'codeBlock', '|',
                'sourceEditing', '|',
            ],
            shouldNotGroupWhenFull: true
        },
        title: {
          placeholder: 'Title'
        },
        placeholder: 'Content'
    };
} );
</script>

<style>
.ck-editor__editable {
  min-height: 500px;
}
</style>
