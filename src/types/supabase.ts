export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json }
  | Json[]

export interface Database {
  public: {
    Tables: {
      chat: {
        Row: {
          created_at: string | null
          id: string
          initial_message: Json
        }
        Insert: {
          created_at?: string | null
          id: string
          initial_message: Json
        }
        Update: {
          created_at?: string | null
          id?: string
          initial_message?: Json
        }
      }
      "chat-message": {
        Row: {
          chat_id: string
          content: Json
          created_at: string | null
          id: number
        }
        Insert: {
          chat_id: string
          content: Json
          created_at?: string | null
          id?: number
        }
        Update: {
          chat_id?: string
          content?: Json
          created_at?: string | null
          id?: number
        }
      }
      "chat-party": {
        Row: {
          chat: string
          created_at: string | null
          id: number
          user: string
        }
        Insert: {
          chat: string
          created_at?: string | null
          id?: number
          user: string
        }
        Update: {
          chat?: string
          created_at?: string | null
          id?: number
          user?: string
        }
      }
      "identity-key": {
        Row: {
          created_at: string | null
          id: string
          key: string
        }
        Insert: {
          created_at?: string | null
          id: string
          key: string
        }
        Update: {
          created_at?: string | null
          id?: string
          key?: string
        }
      }
      "onetime-key": {
        Row: {
          created_at: string | null
          id: number
          key: string
          user: string
        }
        Insert: {
          created_at?: string | null
          id?: number
          key: string
          user: string
        }
        Update: {
          created_at?: string | null
          id?: number
          key?: string
          user?: string
        }
      }
      prekey: {
        Row: {
          created_at: string | null
          id: string
          key: string
          signature: string
        }
        Insert: {
          created_at?: string | null
          id: string
          key: string
          signature: string
        }
        Update: {
          created_at?: string | null
          id?: string
          key?: string
          signature?: string
        }
      }
      profiles: {
        Row: {
          avatar_url: string | null
          full_name: string | null
          id: string
          updated_at: string | null
          username: string | null
        }
        Insert: {
          avatar_url?: string | null
          full_name?: string | null
          id: string
          updated_at?: string | null
          username?: string | null
        }
        Update: {
          avatar_url?: string | null
          full_name?: string | null
          id?: string
          updated_at?: string | null
          username?: string | null
        }
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}
