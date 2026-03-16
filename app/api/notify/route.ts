import { NextRequest, NextResponse } from "next/server";

const GOOGLE_SCRIPT_URL = process.env.GOOGLE_SCRIPT_URL!;

export async function POST(req: NextRequest) {
     try {
          const { email } = await req.json();

          if (!email || !email.includes("@")) {
               return NextResponse.json({ error: "Invalid email" }, { status: 400 });
          }

          const res = await fetch(GOOGLE_SCRIPT_URL, {
               method: "POST",
               headers: { "Content-Type": "application/json" },
               body: JSON.stringify({ email }),
          });

          if (!res.ok) throw new Error("Script error");

          return NextResponse.json({ success: true });
     } catch {
          return NextResponse.json({ error: "Failed to save" }, { status: 500 });
     }
}