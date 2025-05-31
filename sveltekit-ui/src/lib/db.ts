import postgres from 'postgres';

const sql = postgres(import.meta.env.VITE_POSTGRES_ADDRESS);
export default sql;
