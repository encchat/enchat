import {toast} from '@zerodevx/svelte-toast'

export const showError = (message: string) => {
    toast.push(message, { classes: ['toast-error']})
}

export const showSuccess = (message: string) => {
    toast.push(message)
}