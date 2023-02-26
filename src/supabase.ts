import { createClient } from "@supabase/supabase-js";
import type {Database} from 'src/types/supabase'
export const supabaseClient = createClient<Database>(import.meta.env.VITE_SUPABASE_URL, import.meta.env.VITE_SUPABASE_PUBLIC_KEY)