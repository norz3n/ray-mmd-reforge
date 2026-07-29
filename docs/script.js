document.addEventListener('DOMContentLoaded', () => {
    // Intersection Observer for scroll animations
    const fadeElements = document.querySelectorAll('.fade-in');
    
    const appearOptions = {
        threshold: 0.1,
        rootMargin: "0px 0px -50px 0px"
    };

    const appearOnScroll = new IntersectionObserver(function(entries, observer) {
        entries.forEach(entry => {
            if (!entry.isIntersecting) {
                return;
            } else {
                entry.target.classList.add('appear');
                observer.unobserve(entry.target);
            }
        });
    }, appearOptions);

    fadeElements.forEach(element => {
        appearOnScroll.observe(element);
    });

    // Navbar scroll effect
    const navbar = document.querySelector('.navbar');
    window.addEventListener('scroll', () => {
        if (window.scrollY > 50) {
            navbar.style.padding = '1rem 0';
            navbar.style.background = 'rgba(15, 20, 25, 0.95)';
        } else {
            navbar.style.padding = '1.5rem 0';
            navbar.style.background = 'rgba(15, 20, 25, 0.8)';
        }
    });

    // Comparison Sliders Logic
    const sliders = document.querySelectorAll('.comparison-slider');
    sliders.forEach(slider => {
        const input = slider.querySelector('.slider-input');
        const beforeWrapper = slider.querySelector('.img-before-wrapper');
        const sliderLine = slider.querySelector('.slider-line');
        const sliderButton = slider.querySelector('.slider-button');

        input.addEventListener('input', (e) => {
            const value = e.target.value;
            beforeWrapper.style.clipPath = `inset(0 ${100 - value}% 0 0)`;
            sliderLine.style.left = `${value}%`;
            sliderButton.style.left = `${value}%`;
        });
    });

    // Dropdown toggle logic for Download button
    const dropdownToggle = document.getElementById('download-dropdown-btn');
    const dropdownMenu = document.querySelector('.dropdown-menu');

    if (dropdownToggle && dropdownMenu) {
        dropdownToggle.addEventListener('click', (e) => {
            e.preventDefault();
            e.stopPropagation();
            dropdownMenu.classList.toggle('show');
        });
        
        document.addEventListener('click', () => {
            dropdownMenu.classList.remove('show');
        });
    }

    // GitHub API Fetch (automatically fetches the latest release tag)
    const downloadBtn = document.getElementById('download-btn');
    const downloadStable = document.getElementById('download-stable');
    
    if (downloadBtn) {
        const repo = 'norz3n/ray-mmd-reforge';
        
        fetch(`https://api.github.com/repos/${repo}/releases/latest`)
            .then(response => response.json())
            .then(data => {
                if (data.tag_name) {
                    // Main button downloads the stable release
                    downloadBtn.textContent = `Download ${data.tag_name}`;
                    downloadBtn.href = data.html_url; 
                    
                    // Dropdown option explicitly lists stable release
                    if (downloadStable) {
                        downloadStable.textContent = `Stable Release (${data.tag_name})`;
                        downloadStable.href = data.html_url;
                    }
                }
            })
            .catch(error => {
                console.error("Failed to fetch release info:", error);
            });
    }
});
