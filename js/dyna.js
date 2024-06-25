// Récupération des éléments
const acc = document.querySelectorAll(".accordion-title");

// Parcours de chaque élément
acc.forEach(function(item) {
    item.addEventListener("click", function() {
        // Bascule de la visibilité du contenu
        let panel = this.nextElementSibling;
        if (panel.style.display === "flex") {
            panel.style.display = "none";
        } else {
            panel.style.display = "flex";
        }
    });
});
