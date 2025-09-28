import { useEffect } from 'react';

export default function NavbarScrollEffect() {
	useEffect(() => {
		// Add class to body to indicate we're on the homepage
		document.body.classList.add('homepage');

		const handleScroll = () => {
			const navbar = document.querySelector('.navbar');
			if (!navbar) return;

			const scrollTop = window.scrollY;
			const isScrolled = scrollTop > 100;

			if (isScrolled) {
				navbar.classList.add('navbar-scrolled');
				navbar.classList.remove('navbar-transparent');
			} else {
				navbar.classList.add('navbar-transparent');
				navbar.classList.remove('navbar-scrolled');
			}
		};

		// Set initial state after a short delay to ensure DOM is ready
		const initializeNavbar = () => {
			const navbar = document.querySelector('.navbar');
			if (navbar) {
				navbar.classList.add('navbar-transparent');
				handleScroll(); // Check initial scroll position
			}
		};

		// Initialize immediately and also after a short delay
		initializeNavbar();
		const timeoutId = setTimeout(initializeNavbar, 100);

		window.addEventListener('scroll', handleScroll);

		return () => {
			// Clean up: remove homepage class and navbar classes
			document.body.classList.remove('homepage');
			const navbar = document.querySelector('.navbar');
			if (navbar) {
				navbar.classList.remove('navbar-transparent', 'navbar-scrolled');
			}
			window.removeEventListener('scroll', handleScroll);
			clearTimeout(timeoutId);
		};
	}, []);

	return null;
}
