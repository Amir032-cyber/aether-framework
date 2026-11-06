export async function getHealth(){const response=await fetch("/health");if(!response.ok)throw new Error("health check failed");return response.json() as Promise<{status:string;service:string}>;}
