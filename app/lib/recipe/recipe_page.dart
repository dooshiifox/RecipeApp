import 'package:flutter/material.dart';
import 'package:recipeapp/recipe/ingredient.dart';
import 'package:recipeapp/recipe/recipe.dart';
import 'package:recipeapp/recipe/recipe_header.dart';

import '../utils/bouncy_scroll.dart';

class RecipePage extends StatefulWidget {
  final Recipe recipe;

  const RecipePage(this.recipe, {Key? key}) : super(key: key);

  @override
  State<RecipePage> createState() => _RecipePageState();
}

class _RecipePageState extends State<RecipePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      // This means the body can go behind the app bar
      // Necessary for the gradient effect of the recipe
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        elevation: 100,
        foregroundColor: Colors.white,
        backgroundColor: const Color(0xFF3a3d41),
        shape: const RoundedRectangleBorder(
          borderRadius: BorderRadius.vertical(
            bottom: Radius.circular(20),
          ),
        ),
        toolbarHeight: 60,
        leadingWidth: 40,
        leading: IconButton(
          icon: const Icon(Icons.arrow_back_rounded, size: 40),
          padding: const EdgeInsets.fromLTRB(32, 0, 0, 0),
          onPressed: () {
            debugPrint('back button pressed');
          },
        ),
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 24),
            child: IconButton(
              icon: const Icon(Icons.bookmark_border_rounded, size: 40),
              onPressed: () {
                debugPrint('bookmark button pressed');
              },
            ),
          ),
        ],
      ),
      body: BouncyScroll(
        topFactor: 0.2,
        child: Column(
          mainAxisAlignment: MainAxisAlignment.start,
          children: <Widget>[
            // Recipe Information (Image, title, rating, etc.)
            RecipeHeader(widget.recipe),
            const SizedBox(height: 20),
            // Recipe Ingredients + method
            Container(
              decoration: BoxDecoration(
                borderRadius: const BorderRadius.all(Radius.circular(30)),
                color: Colors.black.withOpacity(0.07),
              ),
              // Wrap in a Theme to remove the border colour on the
              // expansion tile when it is expanded
              child: Stack(
                clipBehavior: Clip.none,
                alignment: Alignment.center,
                // fit: StackFit.loose,
                children: [
                  Theme(
                    data:
                        ThemeData().copyWith(dividerColor: Colors.transparent),
                    child: Padding(
                      // Make room for the method button below
                      padding: const EdgeInsets.only(bottom: 45),
                      child: ExpansionTile(
                        iconColor: Colors.black.withOpacity(0.8),
                        title: Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          children: [
                            Icon(
                              Icons.receipt_long_rounded,
                              size: 24,
                              color: Colors.black.withOpacity(0.8),
                            ),
                            Padding(
                              padding: const EdgeInsets.only(left: 8.0),
                              child: Text(
                                'Ingredients',
                                style: TextStyle(
                                  fontSize: 24,
                                  fontWeight: FontWeight.bold,
                                  fontStyle: FontStyle.italic,
                                  color: Colors.black.withOpacity(0.8),
                                  letterSpacing: 0,
                                ),
                              ),
                            ),
                          ],
                        ),
                        children: [
                          // ListView.separated(
                          //   itemBuilder: (_, index) => const Text('hello :3'),
                          //   separatorBuilder: (_, __) => const SizedBox(height: 10),
                          //   itemCount: 5,
                          // )
                          Padding(
                            padding: const EdgeInsets.fromLTRB(24, 12, 24, 18),
                            child: Column(
                              children: widget.recipe.ingredients
                                  .map((ingredient) => Padding(
                                      padding: const EdgeInsets.symmetric(
                                          vertical: 4),
                                      child: RecipeIngredient(ingredient)))
                                  .toList(),
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                  // Method button
                  Positioned(
                    bottom: -20,
                    height: 60,
                    width: MediaQuery.of(context).size.width,
                    child: Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 30),
                      child: Container(
                        decoration: BoxDecoration(
                          gradient: LinearGradient(
                            begin: Alignment.topLeft,
                            end: Alignment.bottomRight,
                            colors: [
                              widget.recipe.tlColor,
                              widget.recipe.brColor
                            ],
                          ),
                          borderRadius:
                              const BorderRadius.all(Radius.circular(30)),
                        ),
                        child: Stack(
                          alignment: Alignment.center,
                          children: [
                            Center(
                              child: Text(
                                'Method',
                                style: TextStyle(
                                  fontSize: 32,
                                  fontWeight: FontWeight.w700,
                                  color: Colors.black.withOpacity(0.7),
                                  letterSpacing: 0,
                                ),
                              ),
                            ),
                            Positioned(
                              right: 24,
                              child: Icon(
                                Icons.navigate_next_rounded,
                                size: 36,
                                color: Colors.black.withOpacity(0.7),
                              ),
                            ),
                          ],
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 60),
            // Feedback button
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 30),
              child: Container(
                decoration: BoxDecoration(
                  borderRadius: const BorderRadius.all(Radius.circular(30)),
                  color: Colors.black.withOpacity(0.09),
                ),
                child: Padding(
                  padding:
                      const EdgeInsets.symmetric(horizontal: 30, vertical: 15),
                  child: Column(
                    children: [
                      Center(
                        child: Text(
                          'Do you not like the recipe?\nDo you want something changed?',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.bold,
                            fontStyle: FontStyle.italic,
                            color: Colors.black.withOpacity(0.8),
                            letterSpacing: 0,
                            height: 1,
                          ),
                          textAlign: TextAlign.center,
                        ),
                      ),
                      const SizedBox(
                        height: 6,
                      ),
                      Center(
                        child: Text(
                          'Click the button below and give us your opinions! We want you to have fun cooking, so every thought counts!',
                          style: TextStyle(
                            fontSize: 16,
                            color: Colors.black.withOpacity(0.8),
                            letterSpacing: 0,
                            height: 1,
                          ),
                          textAlign: TextAlign.center,
                        ),
                      ),
                      const SizedBox(
                        height: 24,
                      ),
                      Container(
                        width: double.infinity,
                        height: 48,
                        decoration: const BoxDecoration(
                          gradient: LinearGradient(
                            begin: Alignment.topLeft,
                            end: Alignment.bottomRight,
                            colors: [
                              Color.fromARGB(255, 243, 152, 207),
                              Color.fromARGB(255, 236, 144, 164)
                            ],
                          ),
                          borderRadius: BorderRadius.all(Radius.circular(24)),
                        ),
                        child: Row(
                          mainAxisAlignment: MainAxisAlignment.center,
                          crossAxisAlignment: CrossAxisAlignment.center,
                          children: const [
                            Padding(
                              padding: EdgeInsets.only(right: 8.0),
                              child: Icon(
                                Icons.campaign_rounded,
                                size: 32,
                                color: Color.fromARGB(255, 51, 26, 26),
                              ),
                            ),
                            Text(
                              'Suggest Feedback',
                              style: TextStyle(
                                fontSize: 20,
                                fontWeight: FontWeight.bold,
                                color: Color.fromARGB(255, 51, 26, 26),
                                letterSpacing: 0,
                              ),
                            ),
                          ],
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
            const SizedBox(height: 20),
          ],
        ),
      ),
    );
  }
}

class RecipeIngredient extends StatelessWidget {
  final Ingredient ingredient;
  final bool checked;
  final bool inCart;

  const RecipeIngredient(
    this.ingredient, {
    this.checked = false,
    this.inCart = false,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        checked
            ? const Icon(
                Icons.check_box_rounded,
                size: 24,
                color: Color(0xFF199a35),
              )
            : Icon(
                Icons.check_box_outline_blank_rounded,
                size: 24,
                color: Colors.black.withOpacity(0.8),
              ),
        Expanded(
          child: Padding(
            padding: const EdgeInsets.only(left: 20.0),
            child: Text(
              ingredient.toString(),
              style: TextStyle(
                fontSize: 20,
                color: Colors.black.withOpacity(0.8),
                letterSpacing: -0.2,
              ),
            ),
          ),
        ),
        Icon(
          inCart
              ? Icons.shopping_basket_rounded
              : Icons.shopping_basket_outlined,
          size: 24,
          color: Colors.black.withOpacity(0.8),
        )
      ],
    );
  }
}
