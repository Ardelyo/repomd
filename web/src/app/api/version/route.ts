import { NextResponse } from 'next/server';

export async function GET() {
  // In a real production app, this could be read from package.json or a build artifact
  // For this local implementation, we'll return a static version that can be manually incremented for testing.
  return NextResponse.json({ 
    version: '1.6.2', 
    timestamp: Date.now(),

    message: "AST structural extraction performance improved by 40%."
  });
}
