import 'package:flutter/material.dart';
import 'package:recipeapp/recipe/ingredient.dart';
import 'package:recipeapp/recipe/recipe.dart';
import 'package:recipeapp/recipe/recipe_page.dart';

void main() {
  runApp(const RecipeApp());
}

class RecipeApp extends StatelessWidget {
  const RecipeApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Nutriblocks Recipe',
      theme: ThemeData(fontFamily: 'Louis George Cafe'),
      home: const ContentAndNavbar(),
    );
  }
}

class ContentAndNavbar extends StatefulWidget {
  const ContentAndNavbar({Key? key}) : super(key: key);

  @override
  State<ContentAndNavbar> createState() => _ContentAndNavbarState();
}

class _ContentAndNavbarState extends State<ContentAndNavbar> {
  // Reorder this to change the order of the items in the navbar.
  final _navbarItemOrder = ['Home', 'Search', 'Cart', 'Feedback'];
  final _navbarItems = {
    'Home': const BottomNavigationBarItem(
      tooltip: 'Home',
      label: '',
      icon: Icon(Icons.home_rounded),
    ),
    'Search': const BottomNavigationBarItem(
      tooltip: 'Search Recipes',
      label: '',
      icon: Icon(Icons.search_rounded),
    ),
    'Cart': const BottomNavigationBarItem(
      tooltip: 'The Cart',
      label: '',
      icon: Icon(Icons.shopping_cart_rounded),
    ),
    'Feedback': const BottomNavigationBarItem(
      tooltip: 'Leave Feedback',
      label: '',
      icon: Icon(Icons.campaign_rounded),
    ),
  };

  int _currentIndex = 0;

  String _currentItem() {
    return _navbarItemOrder[_currentIndex];
  }

  Widget _content() {
    var item = _currentItem();
    switch (item) {
      case 'Home':
        return const Center(child: Text('Home!'));
      case 'Search':
        return const RecipePage(Recipe(
          'Home-made Omelette',
          imageLink: 'assets/images/omelette.jpg',
          ingredients: [
            Ingredient('2 Large Eggs'),
            Ingredient('1 Tablespoon of Unsalted Butter'),
            Ingredient('2 Tablespoons of Grated Cheese'),
            Ingredient('3 or 4 Cherry Tomatoes'),
            Ingredient('2 Tablespoons of Basil or Parsley'),
          ],
          nutrients: ['Vitamin A', 'Iron'],
          rating: 4,
          timeToCook: 15,
          servings: 2,
          tlColor: Color(0xFFa6e8f4),
          brColor: Color(0xFFa6b4f4),
          isWeekly: true,
        ));
      case 'Cart':
        return const Center(child: Text('Cart!'));
      case 'Feedback':
        return const Center(child: Text('Feedback!'));
      default:
        throw Exception('Unknown item: $item');
    }
  }

  @override
  Widget build(BuildContext context) {
    final List<BottomNavigationBarItem> items = _navbarItemOrder.map((i) {
      return _navbarItems[i] as BottomNavigationBarItem;
    }).toList();

    return Scaffold(
      body: Column(
        children: [
          Expanded(child: _content()),
          // A divider to seperate the navbar from the content
          const Divider(
            height: 1,
            color: Color(0xFFC7CED3),
          ),
        ],
      ),
      bottomNavigationBar: BottomNavigationBar(
        type: BottomNavigationBarType.fixed,
        currentIndex: _currentIndex,
        backgroundColor: const Color(0xfff0f5f7),
        selectedItemColor: Colors.black,
        unselectedItemColor: Colors.black.withOpacity(0.50),
        onTap: (value) {
          setState(() => _currentIndex = value);
        },
        items: items,
        iconSize: 36,
        // Hide all labels
        showSelectedLabels: false,
        showUnselectedLabels: false,
      ),
    );
  }
}
