import React, { useEffect, useState } from 'react';
import { useColorMode } from '@docusaurus/theme-common';
import { useThemeConfig } from '@docusaurus/theme-common';
import NavbarLayout from '@theme/Navbar/Layout';
import NavbarContent from '@theme/Navbar/Content';

import './styles.css';

export default function CustomNavbar() {
	const [isScrolled, setIsScrolled] = useState(false);
	const { colorMode } = useColorMode();
	const { navbar } = useThemeConfig();

	useEffect(() => {
		const handleScroll = () => {
			const scrollTop = window.scrollY;
			setIsScrolled(scrollTop > 50);
		};

		window.addEventListener('scroll', handleScroll);
		return () => window.removeEventListener('scroll', handleScroll);
	}, []);

	return (
		<NavbarLayout>
			<div className={`custom-navbar ${isScrolled ? 'navbar-scrolled' : 'navbar-transparent'}`}>
				<NavbarContent />
			</div>
		</NavbarLayout>
	);
}
