// Smooth Scrolling for navigation links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        document.querySelector(this.getAttribute('href')).scrollIntoView({
            behavior: 'smooth'
        });
    });
});

// Intersection Observer for fade-in animations on scroll
const observerOptions = {
    root: null,
    rootMargin: '0px',
    threshold: 0.1
};

const observer = new IntersectionObserver((entries, observer) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
            observer.unobserve(entry.target);
        }
    });
}, observerOptions);

// Select all elements that should fade in
const fadeElements = document.querySelectorAll('.feature-box, .card, .col, .metric');

// Apply initial styles and observe
fadeElements.forEach(el => {
    el.style.opacity = '0';
    el.style.transform = 'translateY(20px)';
    el.style.transition = 'opacity 0.6s ease-out, transform 0.6s ease-out';
    observer.observe(el);
});


// FAQ Accordion Logic
document.querySelectorAll('.accordion-header').forEach(button => {
    button.addEventListener('click', () => {
        const content = button.nextElementSibling;

        // Close others
        document.querySelectorAll('.accordion-content').forEach(c => {
            if (c !== content) c.style.maxHeight = null;
        });

        // Toggle current
        if (content.style.maxHeight) {
            content.style.maxHeight = null;
            button.querySelector('.icon').innerText = '+';
        } else {
            content.style.maxHeight = content.scrollHeight + "px";
            button.querySelector('.icon').innerText = '-';
        }
    });
});