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
        }
        Insert: {
          created_at?: string | null
          id?: string
        }
        Update: {
          created_at?: string | null
          id?: string
        }
      }
      "chat-message": {
        Row: {
          chat_id: string
          content: Json
          created_at: string | null
          id: number
          sender_id: string
        }
        Insert: {
          chat_id: string
          content: Json
          created_at?: string | null
          id?: number
          sender_id: string
        }
        Update: {
          chat_id?: string
          content?: Json
          created_at?: string | null
          id?: number
          sender_id?: string
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
          local_id: number
          used: boolean | null
          user: string
        }
        Insert: {
          created_at?: string | null
          id?: number
          key: string
          local_id: number
          used?: boolean | null
          user: string
        }
        Update: {
          created_at?: string | null
          id?: number
          key?: string
          local_id?: number
          used?: boolean | null
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
      get_onetime_key: {
        Args: {
          user_id: string
        }
        Returns: {
          created_at: string | null
          id: number
          key: string
          local_id: number
          used: boolean | null
          user: string
        }[]
      }
      is_member_of: {
        Args: {
          user_id: string
          chat_id: string
        }
        Returns: boolean
      }
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}
