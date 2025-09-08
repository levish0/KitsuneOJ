import ky from 'ky';
import { API_URL } from './config';

export const Api = ky.create({
	prefixUrl: API_URL,
	headers: {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	},
	credentials: 'include',
	timeout: 10 * 1000,
	retry: 2,
})