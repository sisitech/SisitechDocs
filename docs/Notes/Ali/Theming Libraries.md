## Theming Sisitech Libraries

When building and packing your library make sure you don't have any CSS/SCSS code in your library component scss file. As a matter of fact, is it possible to export our library without its component file since we will not be using encaspulation in this version of the docs. 


## Folder Structure

All our library component code will be in the styles folder created in the **styles** folder and imported into the **styles.scss** file on the root directory.

Using the 7-1 patterned structure, our scss files are arranged in the **styles** folder as below

| Folder Name| Files |
|:------ |:------  |
| Abstracts| variables.scss |
| Core| typography.scss |
| Abstracts| variables.scss |
| Abstracts| variables.scss |
| Abstracts| variables.scss |
| Abstracts| variables.scss |
| Abstracts| variables.scss |

We use ids to define our css selectors. This is because ID selectors have the highest specificity amongst the CSS selectors. Because of the unique nature of the ID attribute definition, we are usually styling a really specific element when we call up the ID in our CSS. 

