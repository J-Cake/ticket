import React from 'react';
import Keycloak from 'keycloak-js';

import { config } from './main.js';

export default class AuthenticationManager {
	private keycloak: Keycloak;

	constructor() {
		this.IsAuthenticated = this.IsAuthenticated.bind(this);
		this.keycloak = new Keycloak({
			clientId: config.clientId,
			realm: config.realm,
			url: config.url
		});
	}

	get display(): string {
		if (!this.keycloak.authenticated)
			throw new Error('Not authenticated');
		return this.keycloak.tokenParsed!.preferred_username;
	}

	useAuthenticated() {
		const [token, setToken] = React.useState(null as typeof this.keycloak.tokenParsed | null);

		React.useEffect(() => {
			this.keycloak.init({ onLoad: 'login-required' })
				.then(auth => {
					if (!auth)
						return;

					setToken(this.keycloak.tokenParsed);
				});
		}, []);

		return token;
	}

	IsAuthenticated(props: { children: React.ReactNode }): React.ReactNode {
		const auth = this.useAuthenticated();

		if (auth)
			return props.children;
		else
			return <button onClick={() => this.keycloak.login({})}>Log In</button>
	}
}