import Database from "@tauri-apps/plugin-sql";

let db: Database | null = null;

async function getDb(): Promise<Database> {
  if (!db) {
    db = await Database.load("sqlite:requests.db");
  }
  return db;
}

export interface SavedRequest {
  id: string;
  name: string;
  method: string;
  url: string;
  headers: string;
  body: string | null;
  created_at: string;
  updated_at: string;
}

export async function loadRequests(): Promise<SavedRequest[]> {
  return (await getDb()).select("SELECT * FROM requests ORDER BY updated_at DESC");
}

export async function saveRequest(req: {
  id: string;
  name: string;
  method: string;
  url: string;
  headers: string;
  body: string | null;
}): Promise<void> {
  await (await getDb()).execute(
    `INSERT INTO requests (id, name, method, url, headers, body)
     VALUES ($1, $2, $3, $4, $5, $6)
     ON CONFLICT(id) DO UPDATE SET
       name = $2, method = $3, url = $4, headers = $5, body = $6,
       updated_at = CURRENT_TIMESTAMP`,
    [req.id, req.name, req.method, req.url, req.headers, req.body]
  );
}

export async function deleteRequest(id: string): Promise<void> {
  await (await getDb()).execute("DELETE FROM requests WHERE id = $1", [id]);
}
