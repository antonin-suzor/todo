import postgres from 'postgres';

const sql = postgres('postgres://root_usr:root_pwd@localhost:5432/root_db');
export default sql;