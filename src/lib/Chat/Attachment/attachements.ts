import { fs, invoke } from "@tauri-apps/api";
import { supabaseClient } from "src/supabase";

class BaseAttachment {
    constructor(protected path: string, protected messageId: number, protected chatId: string) {}
}

export interface PreviewAttachment {
    filename: string;
    id: string;
}

export interface FileInfo {
    filename: string,
    size: number,
    nonce: Array<number>
}

interface EncryptedFileInfo {
    path: string;
    file_info: FileInfo
}

export class AttachementUpload extends BaseAttachment {

    private _encryptedFileInfo: EncryptedFileInfo | null

    async encrypt() {
        console.debug('encrypting...')
        this._encryptedFileInfo =  await invoke<EncryptedFileInfo>('encrypt_file', {inputPath: this.path, messageId: this.messageId, chatId: this.chatId})        
    }

    public async upload(messageDatabaseId: number) {
        if (!this._encryptedFileInfo) throw new Error("File not encrypted yet")
        // I didn't think it through. We save the encrypted file in disk to use less ram (data is being encypted in chunks)
        // However, tauri doesn't seem to provide any fs api for streaming files
        // So we have to read the whole file into memory, which made my endevour pointless
        const buffer = await fs.readBinaryFile(this._encryptedFileInfo.file_info.filename, {
            dir: fs.Dir.Temp
        })
        const {data, error} = await supabaseClient.from('chat-message-attachment').insert({
            info: this._encryptedFileInfo.file_info,
            message: messageDatabaseId,
        }).select('id')
        if (error) {
            console.error(error)
            throw error
        }
        const {data: uploadData, error: uploadError} = await supabaseClient.storage.from('encrypted-files').upload(data[0].id, buffer)
        if (uploadError) throw uploadError
    }

}

export class Attachment extends BaseAttachment {

    private _fileInfo: FileInfo | null = null;

    constructor(private receving: boolean, private tempPath: string, path: string, messageId: number, chatId: string) {
        super(path, messageId, chatId)
    }

    public async decrypt(localMessageId: number) {
        if (!this._fileInfo) throw new Error("File not downloaded yet")
        await invoke('decrypt_and_open', {
            receiving: this.receving,
            info: this._fileInfo,
            inputFilename: this.tempPath,
            outputPath: this.path,
            messageId: localMessageId,
            chatId: this.chatId
        })
    }

    public async download(attachementId: string) {
        const {data, error} = await supabaseClient.storage.from('encrypted-files').download(attachementId)
        if (error) throw new Error("Attachment not found")
        await fs.writeBinaryFile(this.tempPath, await data.arrayBuffer(), {
            dir: fs.Dir.Temp
        })
        const {data: getData, error: getError} = await supabaseClient.from('chat-message-attachment')
            .select('info')
            .eq('id', attachementId)
            .single()
        if (getError) throw new Error("Attachment not found")
        this._fileInfo = getData.info
    }
}